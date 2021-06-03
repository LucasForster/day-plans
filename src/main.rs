mod capacities;
mod districts;
mod graph;
mod io;
mod levels;
mod modes;
mod purposes;
mod trips;

fn main() {
    let districts = districts::load().unwrap();
    let categories = purposes::load().unwrap();
    let trips = trips::load(&categories, &districts).unwrap();
    let levels = levels::load(&categories).unwrap();
    capacities::Capacities::new(&trips, &categories, &levels);
    graph::new(&trips);
}
