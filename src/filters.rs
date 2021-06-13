use super::{
    graph::{Edge, Node},
    purposes::Purpose,
    time_bins::TimeBin,
};

trait Filter: Copy {
    type Param;
    fn new(source: &Node, param: Self::Param) -> Self;
    fn expand(&mut self, edge: &Edge, target: &Node) -> Option<bool>;
}

// LENGTH
#[derive(Clone, Copy)]
pub struct LengthFilter {
    length: usize,
    param: LengthFilterParam,
}
#[derive(Clone, Copy)]
pub struct LengthFilterParam {
    max_length: usize,
    min_length: usize,
}
impl Filter for LengthFilter {
    type Param = LengthFilterParam;
    fn new(_: &Node, param: Self::Param) -> Self {
        LengthFilter { length: 0, param }
    }
    fn expand(&mut self, _: &Edge, _: &Node) -> Option<bool> {
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
    activity: Purpose,
}
impl Filter for FirstActivityFilter {
    type Param = FirstActivityFilterParam;
    fn new(node: &Node, param: Self::Param) -> Self {
        FirstActivityFilter {
            valid: node.purpose == param.activity,
        }
    }
    fn expand(&mut self, _: &Edge, _: &Node) -> Option<bool> {
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
    max_time_bin_count: usize,
    min_time_bin_count: usize,
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
    fn expand(&mut self, _: &Edge, target: &Node) -> Option<bool> {
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
    fn expand(&mut self, _: &Edge, target: &Node) -> Option<bool> {
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
    fn expand(&mut self, edge: &Edge, _: &Node) -> Option<bool> {
        // origin of trip includes first activity but is still compatible with activity cycle
        let index = edge.trip.category.origin as usize;
        let prev = self.activities[index];
        self.activities[index] = true;
        Some(!prev)
    }
}

// CAPACITY
// TODO
