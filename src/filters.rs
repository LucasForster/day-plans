use super::capacities::Capacities;
use super::graph::{Edge, Node};
use super::purposes::Purpose;
use super::time_bins;
use itertools::Itertools;
use std::ops::{Deref};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct FilterParams {
    pub length_range: (usize, usize),
    pub first_activity: &'static [Purpose],
    pub duration_min: u8,
    pub cycle: bool,
}
pub struct Filter {
    params: FilterParams,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    pub capacities: Arc<Capacities>,
}
impl Deref for Filter {
    type Target = FilterParams;
    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

impl Filter {
    pub fn new(params: FilterParams, node: Node, capacities: Arc<Capacities>) -> Result<Self, ()> {
        if !params.first_activity.contains(&node.purpose) {
            return Err(());
        }
        Ok(Filter {
            params,
            nodes: vec![node],
            edges: Vec::new(),
            capacities,
        })
    }

    pub fn to_parent(&mut self) {
        assert!(self.edges.pop().is_some());
        self.nodes.pop();
    }

    pub fn to_child(&mut self, target: &Node, edge: &Edge) -> Result<Option<PotentialPath>, ()> {
        self.nodes.push(*target);
        self.edges.push(*edge);
        match self.check(target, edge) {
            Ok(true) => Ok(Some(PotentialPath {
                nodes: self.nodes.clone(),
                edges: self.edges.clone(),
            })),
            Ok(false) => Ok(None),
            Err(()) => Err(()),
        }
    }
    fn check(&self, target: &Node, edge: &Edge) -> Result<bool, ()> {
        let mut is_valid_path = true;
        macro_rules! check {
            ($check:expr, $name:expr) => {
                if $check.is_none() {
                    is_valid_path = false;
                } else if !$check.unwrap() {
                    // println!($name);
                    return Err(());
                }
            };
        }
        check!(self.check_length(), "length");
        check!(self.check_duration(), "duration");
        check!(self.check_activity_cycle(target), "cycle");
        check!(self.check_trip_capacity(edge), "tripcap");
        check!(self.check_level_capacity(target, edge), "levelcap");
        check!(self.check_mode_capacity(edge), "modecap");
        // if is_valid_path {
        //     println!("valid");
        // }
        Ok(is_valid_path)
    }
    fn check_length(&self) -> Option<bool> {
        if self.nodes.len() < self.length_range.0 {
            None
        } else {
            Some(self.nodes.len() < self.length_range.1)
        }
    }
    fn check_duration(&self) -> Option<bool> {
        let duration: u8 = self
            .nodes
            .iter()
            .zip(self.nodes.iter().skip(1))
            .map(|(first, second)| second.time_bin - first.time_bin)
            .sum();
        if duration > time_bins::COUNT as u8 {
            return Some(false);
        }
        if duration < self.duration_min {
            None
        } else {
            Some(true)
        }
    }
    fn check_activity_cycle(&self, target: &Node) -> Option<bool> {
        if !self.cycle || target.purpose.eq(&self.nodes.first().unwrap().purpose) {
            Some(true)
        } else {
            None
        }
    }
    fn check_trip_capacity(&self, edge: &Edge) -> Option<bool> {
        let prev_count = self
            .edges
            .iter()
            .filter(|other| other.trip.eq(edge.trip))
            .count();
        Some(prev_count < self.capacities.get_trip(edge.trip))
    }
    fn check_level_capacity(&self, target: &Node, edge: &Edge) -> Option<bool> {
        let prev_count = self
            .nodes
            .iter()
            .zip(self.edges.iter())
            .filter(|(other_node, other_edge)| {
                other_node.time_bin == target.time_bin
                    && other_edge.trip.category == edge.trip.category
            })
            .count();
        Some(
            prev_count
                < self
                    .capacities
                    .get_level(edge.trip.category, target.time_bin),
        )
    }
    fn check_mode_capacity(&self, edge: &Edge) -> Option<bool> {
        let prev_count = self
            .edges
            .iter()
            .filter(|other| other.mode.eq(edge.mode))
            .count();
        Some(prev_count < self.capacities.get_mode(edge.mode))
    }
}

pub struct PotentialPath {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}
impl PotentialPath {
    pub fn try_extracting(
        &self,
        capacities: &mut Capacities,
        plans: &mut Vec<Vec<(Node, Edge)>>,
    ) -> Result<(), ()> {
        // check
        let trip_usage = self.edges.iter().map(|edge| edge.trip).counts();
        for trip in trip_usage.keys() {
            if *trip_usage.get(trip).unwrap() > capacities.get_trip(trip) {
                return Err(());
            }
        }
        let level_usage = self
            .edges
            .iter()
            .map(|edge| edge.trip.category)
            .zip(self.nodes.iter().map(|node| node.time_bin))
            .counts();
        for (category, time_bin) in level_usage.keys() {
            if *level_usage.get(&(category, *time_bin)).unwrap()
                > capacities.get_level(category, *time_bin)
            {
                return Err(());
            }
        }
        let mode_usage = self.edges.iter().map(|edge| edge.mode).counts();
        for mode in mode_usage.keys() {
            if *mode_usage.get(mode).unwrap() > capacities.get_mode(mode) {
                return Err(());
            }
        }
        // extract
        for trip in trip_usage.keys() {
            capacities.reduce_trip(trip, *trip_usage.get(trip).unwrap());
        }
        for (category, time_bin) in level_usage.keys() {
            capacities.reduce_level(
                category,
                *time_bin,
                *level_usage.get(&(category, *time_bin)).unwrap(),
            );
        }
        for mode in mode_usage.keys() {
            capacities.reduce_mode(mode, *mode_usage.get(mode).unwrap());
        }
        plans.push(
            self.nodes
                .iter()
                .copied()
                .zip(self.edges.iter().copied())
                .collect(),
        );
        Ok(())
    }
}
