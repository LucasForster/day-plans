use std::collections::HashMap;

use super::levels::{Levels, TimeBin, TimeBins};
use super::modes::Mode;
use super::purposes::{Category, Categories};
use super::trips::{Trip, Trips};

use strum::IntoEnumIterator;


type Count = usize;
type OfTrips<'t> = HashMap<&'t Trip<'t, 't>, Count>;
type OfLevels<'c> = HashMap<(&'c Category, TimeBin), Count>;
type OfModes = HashMap<Mode, Count>;

pub struct Capacities<'c> {
    pub of_trips: OfTrips<'c>,
    pub of_levels: OfLevels<'c>,
    pub of_modes: OfModes,
}
impl Capacities<'_> {
    pub fn new<'c, 'l>(trips: &'c Trips, categories: &'c Categories, levels: &'l Levels) -> Capacities<'c> {
        let mut of_trips: OfTrips<'c> = OfTrips::new();
        {
            for trip in trips.trips.iter() {
                of_trips.insert(&trip, trip.count);
            }
        }
        let mut of_levels: OfLevels<'c> = OfLevels::new();
        {
            let mut generators: HashMap<&'c Category, Generator<TimeBin>> = HashMap::new();
            for category in categories.iter() {
                let mut input: Vec<(TimeBin, Share)> = Vec::new();
                for time_bin in TimeBins {
                    input.push((time_bin, levels.get_level(&category, time_bin)));
                }
                generators.insert(&category, Generator::new(input));
            }
            for trip in trips.trips.iter() {
                let time_bin = generators.get_mut(&trip.category).unwrap().next().unwrap();
                let curr_count: Count = *of_levels.get(&(trip.category, time_bin)).unwrap_or(&0);
                of_levels.insert((trip.category, time_bin), curr_count + 1);
            }
        }
        let mut of_modes: OfModes = OfModes::new();
        {
            let input: Vec<(Mode, Share)> = Mode::iter().map(|mode| (mode, mode.get_share())).collect();
            let mut generator = Generator::new(input);
            for _ in trips.trips.iter() {
                let mode = generator.next().unwrap();
                let curr_count: Count = *of_modes.get(&mode).unwrap_or(&0);
                of_modes.insert(mode, curr_count + 1);
            }
        }
        Capacities {
            of_trips,
            of_levels,
            of_modes,
        }
    }
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
