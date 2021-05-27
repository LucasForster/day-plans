mod districts;
mod io;
mod levels;
mod modes;
mod purposes;
mod trips;

fn main() {
    let districts = districts::load().unwrap();
    let categories = purposes::load().unwrap();
    trips::load(&categories, &districts).unwrap();
    levels::load(&categories).unwrap();
}
