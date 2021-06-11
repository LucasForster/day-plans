mod capacities;
mod categories;
mod districts;
mod filters;
mod graph;
mod io;
mod levels;
mod modes;
mod purposes;
mod trips;

fn main() {
    let levels = levels::load();
    capacities::Capacities::new(&levels);
    graph::Graph::new();
}
