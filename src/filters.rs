use super::{
    capacities::Capacities,
    graph::{Edge, Graph, Node},
    modes,
    purposes::Purpose,
    time_bins::{self, TimeBin},
};
use std::sync::{Arc, RwLockReadGuard};

pub trait Filter: Copy {
    type Param;
    fn new(source: &Node, param: Self::Param) -> Self;
    fn expand(
        &mut self,
        edge: &Edge,
        target: &Node,
        graph: &Arc<Graph>,
        capacities: &RwLockReadGuard<Capacities>,
    ) -> Option<bool>;
}

// LENGTH
#[derive(Clone, Copy)]
pub struct LengthFilter {
    length: usize,
    param: LengthFilterParam,
}
#[derive(Clone, Copy)]
pub struct LengthFilterParam {
    pub max_length: usize,
    pub min_length: usize,
}
impl Filter for LengthFilter {
    type Param = LengthFilterParam;
    fn new(_: &Node, param: Self::Param) -> Self {
        LengthFilter { length: 0, param }
    }
    fn expand(
        &mut self,
        _: &Edge,
        _: &Node,
        _: &Arc<Graph>,
        _: &RwLockReadGuard<Capacities>,
    ) -> Option<bool> {
        self.length += 1;
        match self.length {
            length if length > self.param.max_length => Some(false),
            length if length < self.param.min_length => None,
            _ => Some(true),
        }
    }
}

// FIRST ACTIVITY
#[derive(Clone, Copy)]
pub struct FirstActivityFilter {
    valid: bool,
}
pub struct FirstActivityFilterParam {
    pub activity: Purpose,
}
impl Filter for FirstActivityFilter {
    type Param = FirstActivityFilterParam;
    fn new(node: &Node, param: Self::Param) -> Self {
        FirstActivityFilter {
            valid: node.purpose == param.activity,
        }
    }
    fn expand(
        &mut self,
        _: &Edge,
        _: &Node,
        _: &Arc<Graph>,
        _: &RwLockReadGuard<Capacities>,
    ) -> Option<bool> {
        Some(self.valid)
    }
}

// DURATION
#[derive(Clone, Copy)]
pub struct DurationFilter {
    time_bin_count: usize,
    last_time_bin: TimeBin,
    param: DurationFilterParam,
}
#[derive(Clone, Copy)]
pub struct DurationFilterParam {
    pub max_time_bin_count: usize,
    pub min_time_bin_count: usize,
}
impl Filter for DurationFilter {
    type Param = DurationFilterParam;
    fn new(node: &Node, param: Self::Param) -> Self {
        DurationFilter {
            time_bin_count: 0,
            last_time_bin: node.time_bin,
            param,
        }
    }
    fn expand(
        &mut self,
        _: &Edge,
        target: &Node,
        _: &Arc<Graph>,
        _: &RwLockReadGuard<Capacities>,
    ) -> Option<bool> {
        self.time_bin_count += target.time_bin.value() - self.last_time_bin.value();
        self.last_time_bin = target.time_bin;
        match self.time_bin_count {
            count if count > self.param.max_time_bin_count => Some(false),
            count if count < self.param.min_time_bin_count => None,
            _ => Some(true),
        }
    }
}

// ACTIVITY CYCLE
#[derive(Clone, Copy)]
pub struct ActivityCycleFilter {
    first_activity: Purpose,
}
impl Filter for ActivityCycleFilter {
    type Param = ();
    fn new(node: &Node, _: Self::Param) -> Self {
        ActivityCycleFilter {
            first_activity: node.purpose,
        }
    }
    fn expand(
        &mut self,
        _: &Edge,
        target: &Node,
        _: &Arc<Graph>,
        _: &RwLockReadGuard<Capacities>,
    ) -> Option<bool> {
        if self.first_activity == target.purpose {
            Some(true)
        } else {
            None
        }
    }
}

// DISTINCT ACTIVITIES
#[derive(Clone, Copy)]
pub struct DistinctActivitesFilter {
    activities: [bool; Purpose::COUNT as usize],
}
impl Filter for DistinctActivitesFilter {
    type Param = ();
    fn new(_: &Node, _: Self::Param) -> Self {
        DistinctActivitesFilter {
            activities: [false; Purpose::COUNT as usize],
        }
    }
    fn expand(
        &mut self,
        edge: &Edge,
        _: &Node,
        _: &Arc<Graph>,
        _: &RwLockReadGuard<Capacities>,
    ) -> Option<bool> {
        // origin of trip includes first activity but is still compatible with activity cycle
        let index = edge.trip.category.origin as usize;
        let prev = self.activities[index];
        self.activities[index] = true;
        Some(!prev)
    }
}

// CAPACITY
#[derive(Clone, Copy)]
pub struct CapacityFilter {
    trip_counts: [Option<(usize, usize)>; time_bins::COUNT / 2],
    level_counts: [Option<(usize, TimeBin, usize)>; time_bins::COUNT / 2],
    mode_counts: [usize; modes::COUNT],
}
impl Filter for CapacityFilter {
    type Param = ();
    fn new(_: &Node, _: Self::Param) -> CapacityFilter {
        CapacityFilter {
            trip_counts: [None; time_bins::COUNT / 2],
            level_counts: [None; time_bins::COUNT / 2],
            mode_counts: [0; modes::COUNT],
        }
    }
    fn expand(
        &mut self,
        edge: &Edge,
        node: &Node,
        _: &Arc<Graph>,
        capacities: &RwLockReadGuard<Capacities>,
    ) -> Option<bool> {
        let trip_index = self
            .trip_counts
            .iter()
            .position(|option| option.is_none() || option.unwrap().0 == edge.trip.index)
            .unwrap();
        if self.trip_counts[trip_index].is_none() {
            self.trip_counts[trip_index] = Some((edge.trip.index, 1));
        } else {
            self.trip_counts[trip_index] = {
                let mut prev = self.trip_counts[trip_index].unwrap();
                prev.1 += 1;
                Some(prev)
            };
        }
        let level_index = self
            .level_counts
            .iter()
            .position(|option| {
                option.is_none() || {
                    let level_count = option.unwrap();
                    level_count.0 == edge.trip.category.index && level_count.1 == node.time_bin
                }
            })
            .unwrap();
        if self.level_counts[level_index].is_none() {
            self.level_counts[level_index] = Some((edge.trip.category.index, node.time_bin, 1));
        } else {
            self.level_counts[level_index] = {
                let mut prev = self.level_counts[level_index].unwrap();
                prev.2 += 1;
                Some(prev)
            };
        }
        let mode_index = edge.mode.index;
        self.mode_counts[mode_index] += 1;

        Some(
            capacities.get_trip(edge.trip) >= self.trip_counts[trip_index].unwrap().1
                && capacities.get_level(edge.trip.category, node.time_bin)
                    >= self.level_counts[level_index].unwrap().2
                && capacities.get_mode(edge.mode) >= self.mode_counts[mode_index],
        )
    }
}
