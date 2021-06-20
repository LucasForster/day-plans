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
    let chunk_size = (node_indices.len() as f64 / 100f64).ceil() as usize;

    let mut result: Vec<Vec<&'static Trip>> = Vec::new();
    let mut total_steps: u64 = 0;
    for (chunk_count, chunk) in node_indices.chunks(chunk_size).enumerate() {
        let secs = start.elapsed().unwrap().as_secs();
        println!(
            "{}:{:02}:{:02} {:2}% {:4} plans, {:<7.2e} steps",
            ((secs / 60) / 60) % 60,
            (secs / 60) % 60,
            secs % 60,
            chunk_count,
            result.len(),
            total_steps,
        );
        let mut chunk_result = chunk
            .par_iter()
            .map(|&node_index| execute(graph.clone(), capacities.clone(), node_index))
            .reduce(
                || (Vec::new(), 0u64),
                |(mut acc, total_steps), (mut result, search_steps)| {
                    acc.append(&mut result);
                    (acc, total_steps + search_steps)
                },
            );
        result.append(&mut chunk_result.0);
        total_steps += chunk_result.1;
    }
    println!("Found {} plans.", result.len());
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
) -> (Vec<Vec<&'static Trip>>, u64) {
    let mut capacities_read: RwLockReadGuard<Capacities> = capacities_arc.read().unwrap();
    let mut plans: Vec<Vec<&Trip>> = Vec::new();
    let root_filter = CombinedFilter::new(graph.node(node_index));
    let mut states: Vec<State> = Vec::new();
    let mut search_steps: u64 = 0;

    macro_rules! try_extracting {
        () => {
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
        }
    }

    loop {
        search_steps += 1;
        let to_child: Result<Option<bool>, ()> = (|| {
            let (mut filter, edge_index) = if states.is_empty() {
                match graph.first_edge(node_index) {
                    Some(edge_index) => (root_filter.clone(), edge_index),
                    None => return Err(()),
                }
            } else {
                let state = states.last().unwrap();
                match graph.first_edge(graph.target_index(state.edge_index)) {
                    Some(edge_index) => (state.filter.clone(), edge_index),
                    None => return Err(()),
                }
            };
            let result = filter.expand(edge_index, &graph, &capacities_read);
            states.push(State { edge_index, filter });
            Ok(result)
        })();
        match to_child {
            Ok(None) => continue,
            Ok(Some(true)) => {
                try_extracting!();
                continue; // continue probably unnecessary
            }
            Err(()) => {
                if states.is_empty() {
                    return (plans, search_steps);
                }
            }
            Ok(Some(false)) => {}
        }
        loop {
            search_steps += 1;
            let to_sibling: Result<Option<bool>, ()> = (|| {
                debug_assert!(!states.is_empty());
                let edge_index = match graph.next_edge(states.last().unwrap().edge_index) {
                    Some(edge_index) => edge_index,
                    None => return Err(()),
                };
                let mut filter = if states.len() == 1 {
                    root_filter.clone()
                } else {
                    states.get(states.len() - 2).unwrap().filter.clone()
                };
                let result = filter.expand(edge_index, &graph, &capacities_read);
                *states.last_mut().unwrap() = State { edge_index, filter };
                Ok(result)
            })();
            match to_sibling {
                Err(()) => {
                    states.pop();
                    if states.is_empty() {
                        return (plans, search_steps);
                    } else {
                        continue;
                    }
                }
                Ok(Some(true)) => {
                    try_extracting!();
                    break;
                }
                Ok(None) => break,
                Ok(Some(false)) => continue,
            }
        }
    }
}
