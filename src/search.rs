use super::{
    capacities::Capacities,
    filters::{CombinedFilter, Filter},
    graph::Graph,
    trips::Trip,
};
use petgraph::graph::{EdgeIndex, NodeIndex};
use rayon::prelude::*;
use std::{
    sync::{Arc, RwLock, RwLockReadGuard},
    time::SystemTime,
};

pub fn search() -> Vec<Vec<&'static Trip>> {
    let start = SystemTime::now();

    let graph = Arc::new(Graph::new());
    let capacities = Arc::new(RwLock::new(Capacities::new()));

    let node_indices = graph.node_indices();
    let chunk_size = (node_indices.len() as f64 / 1000f64).ceil() as usize;

    let mut result: Vec<Vec<&'static Trip>> = Vec::new();
    for (chunk_count, chunk) in node_indices.chunks(chunk_size).enumerate() {
        let secs = start.elapsed().unwrap().as_secs();
        println!(
            "{:02}:{:02}:{:02} {:02}.{}% {} plans",
            ((secs / 60) / 60) % 60,
            (secs / 60) % 60,
            secs % 60,
            chunk_count / 10,
            chunk_count % 10,
            result.len()
        );
        result.append(
            &mut chunk
                .par_iter()
                .map(|&node_index| execute(graph.clone(), capacities.clone(), node_index))
                .reduce(
                    || Vec::new(),
                    |mut acc, mut result| {
                        acc.append(&mut result);
                        acc
                    },
                ),
        );
    }
    result
}

struct State {
    edge_index: EdgeIndex,
    filter: CombinedFilter,
}

fn execute(
    graph: Arc<Graph>,
    capacities_arc: Arc<RwLock<Capacities>>,
    node_index: NodeIndex,
) -> Vec<Vec<&'static Trip>> {
    let mut capacities_read: RwLockReadGuard<Capacities> = capacities_arc.read().unwrap();
    let mut plans: Vec<Vec<&Trip>> = Vec::new();
    let root_filter = CombinedFilter::new(graph.node(node_index), ());
    let mut states: Vec<State> = Vec::new();

    loop {
        let to_child: Result<Option<bool>, ()> = {
            if states.is_empty() && graph.first_edge(node_index).is_none() {
                Err(())
            } else {
                let (parent_filter, edge_index) = if states.is_empty() {
                    (root_filter, graph.first_edge(node_index).unwrap())
                } else {
                    let state = states.last().unwrap();
                    (state.filter, state.edge_index)
                };
                let mut filter = parent_filter.clone();
                let result = filter.expand(
                    graph.edge(edge_index),
                    graph.node(graph.target_index(edge_index)),
                    &graph,
                    &capacities_read,
                );
                states.push(State { edge_index, filter });
                Ok(result)
            }
        };
        match to_child {
            Ok(None) => continue,
            Ok(Some(true)) => {
                drop(capacities_read);
                let mut capacities_write = capacities_arc.write().unwrap();
                let capacity_filter = states.last().unwrap().filter.capacity_filter();
                while capacity_filter
                    .try_extracting(&mut capacities_write)
                    .is_ok()
                {
                    plans.push(
                        states
                            .iter()
                            .map(|state| graph.edge(state.edge_index).trip)
                            .collect(),
                    );
                }
                drop(capacities_write);
                capacities_read = capacities_arc.read().unwrap();
                continue; // continue probably unnecessary
            }
            Err(()) => {
                if states.is_empty() {
                    return plans;
                }
            }
            Ok(Some(false)) => {}
        }
        loop {
            let to_sibling: Result<Option<bool>, ()> = {
                let option = graph.next_edge(states.last().unwrap().edge_index);
                if option.is_none() {
                    Err(())
                } else {
                    let edge_index = option.unwrap();
                    let parent_filter = if states.len() == 1 {
                        root_filter
                    } else {
                        states.get(states.len() - 2).unwrap().filter
                    };
                    let mut filter = parent_filter.clone();
                    let result = filter.expand(
                        graph.edge(edge_index),
                        graph.node(graph.target_index(edge_index)),
                        &graph,
                        &capacities_read,
                    );
                    *states.last_mut().unwrap() = State { edge_index, filter };
                    Ok(result)
                }
            };
            match to_sibling {
                Err(()) => {
                    states.pop();
                    if states.is_empty() {
                        return plans;
                    } else {
                        continue;
                    }
                }
                Ok(Some(true)) => {
                    drop(capacities_read);
                    let mut capacities_write = capacities_arc.write().unwrap();
                    let capacity_filter = states.last().unwrap().filter.capacity_filter();
                    while capacity_filter
                        .try_extracting(&mut capacities_write)
                        .is_ok()
                    {
                        plans.push(
                            states
                                .iter()
                                .map(|state| graph.edge(state.edge_index).trip)
                                .collect(),
                        );
                    }
                    drop(capacities_write);
                    capacities_read = capacities_arc.read().unwrap();
                    break;
                }
                Ok(None) => break,
                Ok(Some(false)) => continue,
            }
        }
    }
}
