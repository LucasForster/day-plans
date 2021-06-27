use super::capacities::Capacities;
use super::filters::{self, Filter, PotentialPath};
use super::graph::{Edge, Graph, Node};
use super::purposes::Purpose;
use petgraph::graph::{EdgeIndex, NodeIndex};
use rayon::prelude::*;
use std::sync::Arc;
use std::time::SystemTime;

pub fn search() -> Vec<Vec<(Node, Edge)>> {
    let start = SystemTime::now();

    let mut graph = Graph::new();
    let mut capacities = Capacities::new();

    let node_indices = graph.node_indices();
    let chunk_size = (node_indices.len() as f64 / 10000f64).ceil() as usize;

    let mut plans: Vec<Vec<(Node, Edge)>> = Vec::new();
    let mut total_steps: u64 = 0;
    for (chunk_count, chunk) in node_indices.chunks(chunk_size).enumerate() {
        let secs = start.elapsed().unwrap().as_secs();
        println!(
            "{:2}:{:02}:{:02} {:2}.{:02}% {:4} plans, {:<7.2e} steps",
            ((secs / 60) / 60) % 60,
            (secs / 60) % 60,
            secs % 60,
            chunk_count / 100,
            chunk_count % 100,
            plans.len(),
            total_steps,
        );
        let graph_arc = Arc::new(graph.clone());
        let capacities_arc = Arc::new(capacities.clone());
        let (potential_paths, step_sum) = chunk
            .par_iter()
            .map(|&node_index| execute(graph_arc.clone(), node_index, capacities_arc.clone()))
            .reduce(
                || (Vec::new(), 0u64),
                |(mut acc, sum), (mut potential_paths, steps)| {
                    acc.append(&mut potential_paths);
                    (acc, sum + steps)
                },
            );
        total_steps += step_sum;
        let prev_plan_count = plans.len();
        for potential_path in potential_paths {
            while potential_path
                .try_extracting(&mut capacities, &mut plans)
                .is_ok()
            {}
        }
        if plans.len() > prev_plan_count {
            graph.filter(&capacities);
        }
    }
    println!("Found {} plans.", plans.len());
    plans
}

fn execute(
    graph: Arc<Graph>,
    node_index: NodeIndex,
    capacities: Arc<Capacities>,
) -> (Vec<PotentialPath>, u64) {
    let filter_params = filters::FilterParams {
        length_range: (2..6),
        first_activity: vec![Purpose::Home],
        duration_min: 40,
        capacities,
    };

    let mut plans: Vec<PotentialPath> = Vec::new();
    let mut search_steps: u64 = 0;
    let mut filter = match Filter::new(filter_params, *graph.node(node_index)) {
        Ok(filter) => filter,
        Err(()) => return (plans, search_steps),
    };

    let mut node_indices = vec![node_index];
    let mut edge_indices: Vec<EdgeIndex> = Vec::new();

    macro_rules! unwrap_or_return {
        ($option:expr, $value:expr) => {
            match $option {
                Some(v) => v,
                None => return $value,
            }
        };
    }

    fn to_child(
        search_steps: &mut u64,
        edge_indices: &mut Vec<EdgeIndex>,
        node_indices: &mut Vec<NodeIndex>,
        filter: &mut Filter,
        graph: &Graph,
        plans: &mut Vec<PotentialPath>,
    ) -> bool {
        *search_steps += 1;
        let edge_index = unwrap_or_return!(graph.first_edge(*node_indices.last().unwrap()), false);
        let target_index = graph.target_index(edge_index);
        match filter.to_child(graph.node(target_index), graph.edge(edge_index)) {
            Err(()) => return false,
            Ok(option) => {
                edge_indices.push(edge_index);
                node_indices.push(target_index);
                if option.is_some() {
                    plans.push(option.unwrap());
                }
                return true;
            }
        }
    }

    fn to_sibling(
        search_steps: &mut u64,
        edge_indices: &mut Vec<EdgeIndex>,
        node_indices: &mut Vec<NodeIndex>,
        filter: &mut Filter,
        graph: &Graph,
        plans: &mut Vec<PotentialPath>,
    ) -> bool {
        *search_steps += 1;
        let prev_edge_index = unwrap_or_return!(edge_indices.pop(), false);
        let prev_target_index = node_indices.pop().unwrap();
        let sibling_edge_index = match graph.next_edge(prev_edge_index) {
            Some(sibling) => sibling,
            None => {
                edge_indices.push(prev_edge_index);
                node_indices.push(prev_target_index);
                return false;
            }
        };
        filter.to_parent();
        let sibling_target_index = graph.target_index(sibling_edge_index);
        match filter.to_child(
            graph.node(sibling_target_index),
            graph.edge(sibling_edge_index),
        ) {
            Err(()) => {
                edge_indices.push(prev_edge_index);
                node_indices.push(prev_target_index);
                return false;
            }
            Ok(option) => {
                edge_indices.push(sibling_edge_index);
                node_indices.push(sibling_target_index);
                if option.is_some() {
                    plans.push(option.unwrap());
                }
                return true;
            }
        }
    }

    fn to_parent(
        edge_indices: &mut Vec<EdgeIndex>,
        node_indices: &mut Vec<NodeIndex>,
        filter: &mut Filter,
    ) -> bool {
        unwrap_or_return!(edge_indices.pop(), false);
        node_indices.pop();
        filter.to_parent();
        true
    }

    loop {
        let to_child = to_child(
            &mut search_steps,
            &mut edge_indices,
            &mut node_indices,
            &mut filter,
            &graph,
            &mut plans,
        );
        if to_child {
            continue;
        }
        loop {
            let to_sibling = to_sibling(
                &mut search_steps,
                &mut edge_indices,
                &mut node_indices,
                &mut filter,
                &graph,
                &mut plans,
            );
            if !to_sibling {
                let to_parent = to_parent(&mut edge_indices, &mut node_indices, &mut filter);
                if !to_parent {
                    return (plans, search_steps);
                }
            }
        }
    }
}
