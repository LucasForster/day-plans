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
    let trips = trips::load();
    let levels = levels::load();
    capacities::Capacities::new(&trips, &levels);
    graph::Graph::new(&trips);
}
