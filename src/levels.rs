use std::array;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::{Add, Sub};
use std::time::Duration;

use super::io;
use super::categories::{Id as CategoryId, ID_MAP as CATEGORY_ID_MAP};


type Level = f64;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeBin (usize);
impl Add for TimeBin {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % TimeBins::COUNT)
    }
}
impl Add<Duration> for TimeBin {
    type Output = Self;
    fn add(self, other: Duration) -> Self {
        let seconds = other.as_secs_f64();
        let time_bin_duration = TimeBins::DUR_SECS as f64;
        self + TimeBin((seconds / time_bin_duration).ceil() as usize)
    }
}
impl Sub for TimeBin {
    type Output = usize;
    fn sub(self, other: Self) -> usize {
        if other.0 > self.0 {
            other.0 - self.0
        } else {
            other.0 - self.0 + TimeBins::COUNT
        }
    }
}

pub struct TimeBins;
impl TimeBins {
    pub const COUNT: usize = 48;
    const DUR_SECS: usize = 30*60;
    // TODO: const initialization? macro?
    const TIME_BINS: [TimeBin; TimeBins::COUNT] = [TimeBin(0), TimeBin(1), TimeBin(2), TimeBin(3), TimeBin(4), TimeBin(5), TimeBin(6), TimeBin(7), TimeBin(8), TimeBin(9), TimeBin(10), TimeBin(11), TimeBin(12), TimeBin(13), TimeBin(14), TimeBin(15), TimeBin(16), TimeBin(17), TimeBin(18), TimeBin(19), TimeBin(20), TimeBin(21), TimeBin(22), TimeBin(23), TimeBin(24), TimeBin(25), TimeBin(26), TimeBin(27), TimeBin(28), TimeBin(29), TimeBin(30), TimeBin(31), TimeBin(32), TimeBin(33), TimeBin(34), TimeBin(35), TimeBin(36), TimeBin(37), TimeBin(38), TimeBin(39), TimeBin(40), TimeBin(41), TimeBin(42), TimeBin(43), TimeBin(44), TimeBin(45), TimeBin(46), TimeBin(47)];
}
impl IntoIterator for TimeBins {
    type Item = TimeBin;
    type IntoIter = array::IntoIter<Self::Item, 48>; // TODO: replace 48 with TimeBins::COUNT

    fn into_iter(self) -> Self::IntoIter {
        array::IntoIter::new(Self::TIME_BINS)
    }
}

#[derive(Debug)]
pub struct Levels {
    map: HashMap<CategoryId, [Level; TimeBins::COUNT]>,
}
impl Levels {
    pub fn get_level(&self, category: CategoryId, time_bin: TimeBin) -> Level {
        self.map.get(&category).unwrap()[time_bin.0]
    }
}


pub fn load<'c>() -> Levels {
    let mut map: HashMap<CategoryId, [Level; TimeBins::COUNT]> = HashMap::new();
    for category_id in CATEGORY_ID_MAP.keys() {
        let path = format!("verkehrsfluss/verkehrsflussdaten/pegel{}.txt", category_id.value());
        let record = &io::read_csv(path, true, false, b';', Some(b'/'))[0];
        let mut levels: Vec<Level> = Vec::new();
        for time_bin in TimeBins {
            levels.push(record[time_bin.0].parse().unwrap());
        }
        let sum: Level = levels.iter().fold(0.0, |sum, level| sum + level);
        let norm: Vec<Level> = levels.iter().map(|level| level / sum).collect();
        let array: [Level; TimeBins::COUNT] = norm.try_into().unwrap();
        map.insert(*category_id, array);
    }
    println!("Loaded {} levels, each for {} time bins.", map.len(), TimeBins::COUNT);
    Levels {
        map
    }
}
