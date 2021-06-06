use std::collections::HashMap;

use petgraph::graph::{Graph as Petgraph, NodeIndex};

use super::districts::Id as DistrictId;
use super::levels::{TimeBin, TimeBins};
use super::modes::{Mode, Modes};
use super::purposes::Purpose;
use super::trips::Trip;


type Graph<'t> = Petgraph::<N<'t>, E<'t>>;
pub type N<'t> = (&'t DistrictId, Purpose, TimeBin);
pub type E<'t> = (&'t Trip<'t, 't>, Mode);

pub fn new<'t>(trips: &'t Vec<Trip>) -> Graph<'t> {
    let mut graph = Graph::new();
    let mut nodes = HashMap::<N<'t>, NodeIndex>::new();
    for trip in trips {
        for time_bin in TimeBins {
            for mode in Modes {
                let source_key: N<'t> = (trip.origin, trip.category.origin, time_bin);
                let source_index: NodeIndex = *nodes.entry(source_key)
                    .or_insert(graph.add_node(source_key));

                let destination_time_bin = time_bin + trip.category.origin.duration(); // TODO: leg duration

                let destination_key: N<'t> = (trip.destination, trip.category.destination, destination_time_bin);
                let destination_index: NodeIndex = *nodes.entry(destination_key)
                    .or_insert(graph.add_node(destination_key));

                let edge_key: E<'t> = (&trip, mode);
                graph.add_edge(source_index, destination_index, edge_key);
            }
        }
    }
    graph
}
