mod capacities;
mod districts;
mod filters;
mod graph;
mod io;
mod levels;
mod modes;
mod purposes;
mod trips;

fn main() {
    let categories = purposes::load();
    let trips = trips::load(&categories);
    let levels = levels::load(&categories);
    capacities::Capacities::new(&trips, &categories, &levels);
    graph::Graph::new(&trips);
}
