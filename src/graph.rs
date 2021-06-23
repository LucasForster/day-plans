use super::districts;
use super::modes::{Mode, MODES};
use super::purposes::Purpose;
use super::time_bins::{TimeBin, TIME_BINS};
use super::trips::{Trip, TRIPS};
use petgraph::graph::{EdgeIndex, Graph as Petgraph, NodeIndex};
use petgraph::Direction::Outgoing;
use std::collections::HashMap;

pub struct Graph(Petgraph<Node, Edge>);
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Node {
    pub district_id: districts::Id,
    pub purpose: Purpose,
    pub time_bin: TimeBin,
}
pub struct Edge {
    pub trip: &'static Trip,
    pub mode: &'static Mode,
}

impl Graph {
    pub fn new() -> Self {
        let mut graph = Petgraph::<Node, Edge>::new();
        let mut nodes = HashMap::<Node, NodeIndex>::new();
        for trip in TRIPS.iter() {
            for time_bin in TIME_BINS.into_iter().copied() {
                for mode in MODES.iter() {
                    let trip_category = trip.category;
                    let source_key = Node {
                        district_id: trip.origin.id,
                        purpose: trip_category.origin,
                        time_bin,
                    };
                    if !nodes.contains_key(&source_key) {
                        nodes.insert(source_key, graph.add_node(source_key));
                    }
                    let source_index: NodeIndex = *nodes.get(&source_key).unwrap();

                    let destination_key = Node {
                        district_id: trip.destination.id,
                        purpose: trip_category.destination,
                        time_bin: time_bin + trip_category.origin.duration(), // TODO: leg duration
                    };
                    if !nodes.contains_key(&destination_key) {
                        nodes.insert(destination_key, graph.add_node(destination_key));
                    }
                    let destination_index: NodeIndex = *nodes.get(&destination_key).unwrap();

                    let edge_key = Edge { trip, mode };
                    graph.add_edge(source_index, destination_index, edge_key);
                }
            }
        }
        graph.shrink_to_fit();
        println!("\rBuilt graph: {} nodes, {:.2e} edges", graph.node_count(), graph.edge_count());
        Graph(graph)
    }
    pub fn node_indices(&self) -> Vec<NodeIndex> {
        self.0.node_indices().collect()
    }
    pub fn node(&self, node_index: NodeIndex) -> &Node {
        self.0.node_weight(node_index).unwrap()
    }
    pub fn edge(&self, edge_index: EdgeIndex) -> &Edge {
        self.0.edge_weight(edge_index).unwrap()
    }
    pub fn first_edge(&self, node_index: NodeIndex) -> Option<EdgeIndex> {
        self.0.first_edge(node_index, Outgoing)
    }
    pub fn next_edge(&self, edge_index: EdgeIndex) -> Option<EdgeIndex> {
        self.0.next_edge(edge_index, Outgoing)
    }
    pub fn source_index(&self, edge_index: EdgeIndex) -> NodeIndex {
        self.0.edge_endpoints(edge_index).unwrap().0
    }
    pub fn target_index(&self, edge_index: EdgeIndex) -> NodeIndex {
        self.0.edge_endpoints(edge_index).unwrap().1
    }
}
