use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Once;

use super::io;
use super::purposes::{Category, Categories};


struct TimeBin (usize);
impl TimeBin {
    const COUNT: usize = 48;
    fn new(time_bin: usize) -> TimeBin {
        if time_bin >= TimeBin::COUNT {
            panic!("Unknown time bin {}!", time_bin);
        } else {
            TimeBin(time_bin)
        }
    }
}
// TODO: impl Iterator for TimeBin (but next() signature forces mut on &self?!)

#[derive(Debug)]
pub struct Levels<'c> {
    map: HashMap<&'c Category, [f64; TimeBin::COUNT]>,
}
impl Levels<'_> {
    pub fn get_level(&self, category: &Category, time_bin: TimeBin) -> f64 {
        self.map.get(&category).unwrap()[time_bin.0]
    }
}


const LOAD: Once = Once::new();
pub fn load<'c>(categories: &'c Categories) -> Option<Levels> {
    let mut levels = None;
    LOAD.call_once(|| {
        levels = Some(load_file(categories));
    });
    levels
}
fn load_file<'c>(categories: &'c Categories) -> Levels<'c> {
    let mut map: HashMap<&'c Category, [f64; TimeBin::COUNT]> = HashMap::new();
    for category in categories.iter() {
        let path = format!("verkehrsfluss/verkehrsflussdaten/pegel{}.txt", category.id);
        let record = &io::read_csv(path, true, false, b';', Some(b'/'))[0];
        let mut levels: Vec<f64> = Vec::new();
        for i in 0..TimeBin::COUNT {
            levels.push(record[i].parse().unwrap());
        }
        let sum: f64 = levels.iter().fold(0f64, |sum, level| sum + level);
        let norm: Vec<f64> = levels.iter().map(|level| level / sum).collect();
        let array: [f64; TimeBin::COUNT] = norm.try_into().unwrap();
        map.insert(category, array);
    }
    Levels {
        map
    }
}
