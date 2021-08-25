use super::capacities::Capacities;
use super::filters::{Filter, FilterParams, PotentialPath};
use super::graph::{Edge, Graph, Node};
use super::purposes::Purpose;
use petgraph::graph::{EdgeIndex, NodeIndex};
use rayon::prelude::*;
use std::sync::Arc;
use std::time::SystemTime;

const NUMBER_OF_CHUNKS: usize = 100;
const FILTER_PARAMS: [FilterParams; 1] = [FilterParams {
    length_range: (2, 6),
    first_activity: &[Purpose::Home],
    duration_min: 40,
    cycle: true,
}];

pub fn search() -> Vec<Vec<(Node, Edge)>> {
    let start = SystemTime::now();

    let mut graph_arc = Arc::new(Graph::new());
    let mut capacities_arc = Arc::new(Capacities::new());
    let mut plans: Vec<Vec<(Node, Edge)>> = Vec::new();
    let mut total_steps: u64 = 0;

    for (filter_index, filter_params) in FILTER_PARAMS.iter().enumerate() {
        println!("\t--- STAGE {} ---", filter_index + 1);
        let node_indices: Vec<NodeIndex> = graph_arc
            .node_indices()
            .into_iter()
            .filter(|&node_index| {
                filter_params
                    .first_activity
                    .iter()
                    .find(|&purpose| graph_arc.node(node_index).purpose.eq(purpose))
                    .is_some()
            })
            .collect();
        let chunk_size = (node_indices.len() as f64 / (NUMBER_OF_CHUNKS as f64)).ceil() as usize;

        for (chunk_count, chunk) in node_indices.chunks(chunk_size).enumerate() {
            let secs = start.elapsed().unwrap().as_secs();
            println!(
                "{:02}:{:02}:{:02} {:2.*}% {:4} plans, {:<7.2e} steps",
                ((secs / 60) / 60) % 60,
                (secs / 60) % 60,
                secs % 60,
                ((NUMBER_OF_CHUNKS / 100) as f64).log10().ceil() as usize,
                (100 * chunk_count) as f64 / NUMBER_OF_CHUNKS as f64,
                plans.len(),
                total_steps,
            );

            let (potential_paths, step_sum) = chunk
                .par_iter()
                .map(|&node_index| execute(graph_arc.clone(), node_index, capacities_arc.clone(), filter_params))
                .reduce(
                    || (Vec::new(), 0u64),
                    |(mut acc, sum), (mut potential_paths, steps)| {
                        acc.append(&mut potential_paths);
                        (acc, sum + steps)
                    },
                );
            total_steps += step_sum;

            let prev_plan_count = plans.len();
            let mut capacities = match Arc::try_unwrap(capacities_arc) {
                Ok(capacities) => capacities,
                Err(_) => panic!(),
            };
            for potential_path in potential_paths {
                while potential_path
                    .try_extracting(&mut capacities, &mut plans)
                    .is_ok()
                {}
            }
            capacities_arc = Arc::new(capacities);
            if plans.len() > prev_plan_count {
                let mut graph = match Arc::try_unwrap(graph_arc) {
                    Ok(graph) => graph,
                    Err(_) => panic!(),
                };
                graph.filter_edges(&capacities_arc);
                graph_arc = Arc::new(graph);
            }
        }
        print!("Plan lengths: ");
        for i in 1..10 {
            let count = plans.iter().filter(|&plan| plan.len() == i).count();
            print!("{}: {} | ", i, count);
        }
        println!();
    }
    println!("Found {} plans.", plans.len());
    plans
}

fn execute(
    graph: Arc<Graph>,
    node_index: NodeIndex,
    capacities: Arc<Capacities>,
    filter_params: &FilterParams,
) -> (Vec<PotentialPath>, u64) {
    let mut plans: Vec<PotentialPath> = Vec::new();
    let mut search_steps: u64 = 0;
    let mut filter = match Filter::new(*filter_params, *graph.node(node_index), capacities) {
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
        edge_indices.push(edge_index);
        node_indices.push(target_index);
        match filter.to_child(graph.node(target_index), graph.edge(edge_index)) {
            Err(()) => return false,
            Ok(option) => {
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
    ) -> Result<bool, ()> {
        *search_steps += 1;
        let prev_edge_index = unwrap_or_return!(edge_indices.pop(), Err(()));
        let prev_target_index = node_indices.pop().unwrap();
        let sibling_edge_index = match graph.next_edge(prev_edge_index) {
            Some(sibling) => sibling,
            None => {
                edge_indices.push(prev_edge_index);
                node_indices.push(prev_target_index);
                return Err(());
            }
        };
        let sibling_target_index = graph.target_index(sibling_edge_index);
        edge_indices.push(sibling_edge_index);
        node_indices.push(sibling_target_index);

        filter.to_parent();
        match filter.to_child(
            graph.node(sibling_target_index),
            graph.edge(sibling_edge_index),
        ) {
            Err(()) => Ok(false),
            Ok(option) => {
                if option.is_some() {
                    plans.push(option.unwrap());
                }
                Ok(true)
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
            match to_sibling {
                Ok(true) => break,
                Ok(false) => continue,
                Err(()) => {
                    let to_parent = to_parent(&mut edge_indices, &mut node_indices, &mut filter);
                    if !to_parent {
                        return (plans, search_steps);
                    }
                }
            }
        }
    }
}
