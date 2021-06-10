use std::collections::HashMap;

use petgraph::graph::{Graph as Petgraph, NodeIndex};

use super::categories::ID_MAP as CATEGORY_ID_MAP;
use super::districts::Id as DistrictId;
use super::levels::{TimeBin, TimeBins};
use super::modes::{Mode, Modes};
use super::purposes::Purpose;
use super::trips::Trip;


pub struct Graph<'t>(Petgraph::<Node, Edge<'t>>);
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Node {
    pub district_id: DistrictId,
    pub purpose: Purpose,
    pub time_bin: TimeBin,
}
pub struct Edge<'t> {
    pub trip: &'t Trip,
    pub mode: Mode,
}

impl<'t> Graph<'t> {
    pub fn new(trips: &'t Vec<Trip>) -> Self {
        let mut graph = Petgraph::<Node, Edge<'t>>::new();
        let mut nodes = HashMap::<Node, NodeIndex>::new();
        for trip in trips {
            for time_bin in TimeBins {
                for mode in Modes {
                    let trip_category = CATEGORY_ID_MAP.get(&trip.category_id).unwrap();
                    let source_key = Node {
                        district_id: trip.origin,
                        purpose: trip_category.origin,
                        time_bin,
                    };
                    let source_index: NodeIndex = *nodes.entry(source_key)
                        .or_insert(graph.add_node(source_key));

                    let destination_time_bin = time_bin + trip_category.origin.duration(); // TODO: leg duration

                    let destination_key = Node {
                        district_id: trip.destination,
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
