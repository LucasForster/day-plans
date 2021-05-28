mod capacities;
mod districts;
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
    let capacities = capacities::Capacities::new(&trips, &categories, &levels);
    println!("{:?}", capacities.of_modes);
}
