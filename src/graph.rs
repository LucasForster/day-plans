use super::{
    categories::CATEGORIES,
    trips::Trip, trips::TRIPS,
};

use std::collections::HashMap;

use petgraph::graph::{Graph as Petgraph, NodeIndex};

use super::districts::Id as DistrictId;
use super::levels::{TimeBin, TimeBins};
use super::modes::{Mode, Modes};
use super::purposes::Purpose;


pub struct Graph(Petgraph::<Node, Edge>);
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Node {
    pub district_id: DistrictId,
    pub purpose: Purpose,
    pub time_bin: TimeBin,
}
pub struct Edge {
    pub trip: &'static Trip,
    pub mode: Mode,
}

impl<'t> Graph {
    pub fn new() -> Self {
        let mut graph = Petgraph::<Node, Edge>::new();
        let mut nodes = HashMap::<Node, NodeIndex>::new();
        for trip in TRIPS.iter() {
            for time_bin in TimeBins {
                for mode in Modes {
                    let trip_category = trip.category;
                    let source_key = Node {
                        district_id: trip.origin.id,
                        purpose: trip_category.origin,
                        time_bin,
                    };
                    let source_index: NodeIndex = *nodes.entry(source_key)
                        .or_insert(graph.add_node(source_key));

                    let destination_time_bin = time_bin + trip_category.origin.duration(); // TODO: leg duration

                    let destination_key = Node {
                        district_id: trip.destination.id,
                        purpose: trip_category.destination,
                        time_bin: time_bin + destination_time_bin,
                    };
                    let destination_index: NodeIndex = *nodes.entry(destination_key)
                        .or_insert(graph.add_node(destination_key));

                    let edge_key = Edge {
                        trip,
                        mode,
                    };
                    graph.add_edge(source_index, destination_index, edge_key);
                }
            }
        }
        Graph(graph)
    }
    pub fn node_indices(&self) -> Vec<NodeIndex> {
        self.0.node_indices().collect()
    }
    pub fn node(&self, node_index: NodeIndex) -> &Node {
        self.0.node_weight(node_index).unwrap()
    }
}
