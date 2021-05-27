use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Once;

use super::io;
use super::purposes::{Category, Categories};


type Level = f64;

pub struct TimeBin (usize);
pub struct TimeBins;
impl TimeBins {
    const COUNT: usize = 48;
    // TODO: const initialization? macro?
    const TIME_BINS: [TimeBin; TimeBins::COUNT] = [TimeBin(0), TimeBin(1), TimeBin(2), TimeBin(3), TimeBin(4), TimeBin(5), TimeBin(6), TimeBin(7), TimeBin(8), TimeBin(9), TimeBin(10), TimeBin(11), TimeBin(12), TimeBin(13), TimeBin(14), TimeBin(15), TimeBin(16), TimeBin(17), TimeBin(18), TimeBin(19), TimeBin(20), TimeBin(21), TimeBin(22), TimeBin(23), TimeBin(24), TimeBin(25), TimeBin(26), TimeBin(27), TimeBin(28), TimeBin(29), TimeBin(30), TimeBin(31), TimeBin(32), TimeBin(33), TimeBin(34), TimeBin(35), TimeBin(36), TimeBin(37), TimeBin(38), TimeBin(39), TimeBin(40), TimeBin(41), TimeBin(42), TimeBin(43), TimeBin(44), TimeBin(45), TimeBin(46), TimeBin(47)];
    pub fn iter() -> std::slice::Iter<'static, TimeBin> {
        TimeBins::TIME_BINS.iter()
    }
}

#[derive(Debug)]
pub struct Levels<'c> {
    map: HashMap<&'c Category, [Level; TimeBins::COUNT]>,
}
impl Levels<'_> {
    pub fn get_level(&self, category: &Category, time_bin: TimeBin) -> Level {
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
    let mut map: HashMap<&'c Category, [Level; TimeBins::COUNT]> = HashMap::new();
    for category in categories.iter() {
        let path = format!("verkehrsfluss/verkehrsflussdaten/pegel{}.txt", category.id);
        let record = &io::read_csv(path, true, false, b';', Some(b'/'))[0];
        let mut levels: Vec<Level> = Vec::new();
        for time_bin in TimeBins::iter() {
            levels.push(record[time_bin.0].parse().unwrap());
        }
        let sum: Level = levels.iter().fold(0.0, |sum, level| sum + level);
        let norm: Vec<Level> = levels.iter().map(|level| level / sum).collect();
        let array: [Level; TimeBins::COUNT] = norm.try_into().unwrap();
        map.insert(category, array);
    }
    Levels {
        map
    }
}
