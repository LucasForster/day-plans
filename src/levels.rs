use super::{categories::Category, categories::CATEGORIES, io, time_bins};
use lazy_static::lazy_static;

type Levels = [f64; time_bins::COUNT];

lazy_static! {
    static ref LEVELS_VEC: Vec<Levels> = load();
}

pub fn get_levels(category: &Category) -> Levels {
    LEVELS_VEC[category.index]
}

fn load() -> Vec<Levels> {
    let mut vec: Vec<Levels> = Vec::new();
    for category in CATEGORIES.iter() {
        let path = format!("verkehrsfluss/verkehrsflussdaten/pegel{}.txt", category.id);
        let record = &io::read_csv(path, true, false, b';', Some(b'/'))[0];
        let mut values = [0f64; time_bins::COUNT];
        for (i, entry) in record.iter().take(time_bins::COUNT).enumerate() {
            values[i] = entry.parse().unwrap();
        }
        vec.push(values);
    }
    println!(
        "Loaded {} levels, each for {} time bins.",
        vec.len(),
        time_bins::COUNT
    );
    vec
}
