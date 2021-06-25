use super::graph::{Edge, Node};
use super::purposes::Purpose;
use super::time_bins;
use std::ops::{Deref, Range};

pub struct Params {
    pub length_range: Range<usize>,
    pub first_activity: Vec<Purpose>,
    pub duration_min: u8,
}
pub struct Filter {
    params: Params,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}
impl Deref for Filter {
    type Target = Params;
    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

impl Filter {
    pub fn new(params: Params, node: Node) -> Option<Self> {
        if !params.first_activity.contains(&node.purpose) {
            return None;
        }
        Some(Filter {
            params,
            nodes: vec![node],
            edges: Vec::new(),
        })
    }

    pub fn to_parent(&mut self) {
        assert!(self.edges.pop().is_some());
        self.nodes.pop();
    }

    pub fn to_child(&mut self, target: &Node, edge: &Edge) -> Result<bool, ()> {
        match self.check(target, edge) {
            Some(false) => Err(()),
            v => {
                self.nodes.push(*target);
                self.edges.push(*edge);
                Ok(v.is_some())
            }
        }
    }
    fn check(&self, target: &Node, edge: &Edge) -> Option<bool> {
        let mut is_valid_path = true;
        macro_rules! check {
            ($check:expr) => {
                if $check.is_none() {
                    is_valid_path = false;
                } else if !$check.unwrap() {
                    return Some(false);
                }
            };
        }
        check!(self.check_length());
        check!(self.check_duration(target));
        check!(self.check_activity_cycle(target));
        check!(self.check_distinct_activities(target));
        if is_valid_path {
            Some(true)
        } else {
            None
        }
    }
    fn check_length(&self) -> Option<bool> {
        if self.nodes.len() < self.length_range.start {
            None
        } else {
            Some(self.nodes.len() < self.length_range.end)
        }
    }
    fn check_duration(&self, target: &Node) -> Option<bool> {
        let duration = target.time_bin - self.nodes.first().unwrap().time_bin;
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
        if target.purpose.eq(&self.nodes.first().unwrap().purpose) {
            Some(true)
        } else {
            None
        }
    }
    fn check_distinct_activities(&self, target: &Node) -> Option<bool> {
        Some(
            self.nodes
                .iter()
                .find(|node| node.purpose.eq(&target.purpose))
                .is_none(),
        )
    }
    pub fn try_extracting(&self) -> Vec<(Node, Edge)> {
        self.nodes
            .iter()
            .copied()
            .zip(self.edges.iter().copied())
            .collect()
    }
}
