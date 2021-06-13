mod capacities;
mod categories;
mod districts;
mod filters;
mod graph;
mod io;
mod levels;
mod modes;
mod purposes;
mod time_bins;
mod trips;

fn main() {
    capacities::Capacities::new();
    graph::Graph::new();
}
