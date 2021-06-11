use super::{
    categories::Category, categories::CATEGORIES,
    trips::Trip, trips::TRIPS,
};

use std::collections::HashMap;

use super::levels::{Levels, TimeBin, TimeBins};
use super::modes::{Mode, Modes};


type Count = usize;
type OfLevels = HashMap<(Category, TimeBin), Count>;
type OfModes = HashMap<Mode, Count>;

pub struct Capacities {
    of_trips: Vec<Count>,
    of_levels: Vec<[Count; TimeBins::COUNT]>,
    pub of_modes: OfModes,
}
impl Capacities {
    pub fn new<'l>(levels: &'l Levels) -> Capacities {
        let mut of_levels: Vec<[Count; TimeBins::COUNT]> = Vec::new();
        {
            for category in CATEGORIES.iter() {
                assert!(category.index == of_levels.len());
                let mut generator = Generator::new(
                    TimeBins.into_iter().map(|time_bin| (time_bin, levels.get_level(category, time_bin))).collect());
                let mut counts = [0 as Count; TimeBins::COUNT];
                let total_trip_count: usize = TRIPS.iter().filter(|&trip| trip.category.eq(category)).map(|trip| trip.count).sum();
                for _ in 0..total_trip_count {
                    counts[generator.next().unwrap().value()] += 1;
                }
            }
        }
        let mut of_modes: OfModes = OfModes::new();
        {
            let input: Vec<(Mode, Share)> = Modes.into_iter().map(|mode| (mode, mode.share())).collect();
            let mut generator = Generator::new(input);
            for _ in TRIPS.iter() {
                let mode = generator.next().unwrap();
                let curr_count: Count = *of_modes.get(&mode).unwrap_or(&0);
                of_modes.insert(mode, curr_count + 1);
            }
        }
        Capacities {
            of_trips: TRIPS.iter().map(|trip| trip.count).collect(),
            of_levels,
            of_modes,
        }
    }
    pub fn of_trip(&self, trip: &Trip) -> Count {
        self.of_trips[trip.index]
    }
    pub fn of_level(&self, category: &Category, time_bin: TimeBin) -> Count {
        self.of_levels[category.index][time_bin.value()]
    }
    // TODO: reduce
}


type Share = f64;

struct Generator<T: Copy> {
    elements: Vec<T>,
    distribution: Vec<Share>,
    counts: Vec<usize>,
}
impl<T: Copy> Generator<T> {
    fn new(input: Vec<(T, Share)>) -> Generator<T> {
        Generator {
            elements: input.iter().map(|entry| entry.0).collect(),
            distribution: input.iter().map(|entry| entry.1).collect(),
            counts: vec![0; input.len()],
        }
    }
}
impl<T: Copy> Iterator for Generator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            return None;
        }
        let total: usize = self.counts.iter().sum();
        let target_counts: Vec<f64> = self.distribution.iter().map(|share| share * ((total + 1) as f64)).collect();
        let mut index_max: usize = 0;
        let mut diff_max: f64 = target_counts[index_max] - (self.counts[index_max] as f64);
        for i in 1..self.elements.len() {
            let diff_i = target_counts[i] - (self.counts[i] as f64);
            if diff_i > diff_max {
                index_max = i;
                diff_max = diff_i;
            }
        }
        self.counts[index_max] += 1;
        Some(self.elements[index_max])
    }
}
