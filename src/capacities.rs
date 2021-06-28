use super::categories::{Category, CATEGORIES};
use super::levels;
use super::modes::{Mode, MODES};
use super::time_bins::{self, TimeBin};
use super::trips::{Trip, TRIPS};
use std::convert::TryInto;

type Count = usize;

pub struct Capacities {
    of_trips: Vec<Count>,
    of_levels: Vec<[Count; time_bins::COUNT]>,
    of_modes: Vec<Count>,
}
impl Capacities {
    pub fn new() -> Capacities {
        let of_trips = TRIPS.iter().map(|trip| trip.count).collect();
        let mut of_levels: Vec<[Count; time_bins::COUNT]> = Vec::new();
        for category in CATEGORIES.iter() {
            let total = TRIPS
                .iter()
                .filter(|&trip| trip.category.eq(category))
                .count() as f64;
            let values = levels::get_levels(category)
                .iter()
                .map(|share| share * total)
                .collect::<Vec<f64>>();
            of_levels.push(sum_safe_round(&values).try_into().unwrap());
        }
        let of_modes: Vec<Count> = sum_safe_round(
            &MODES
                .iter()
                .map(|mode| mode.share * (TRIPS.len() as f64))
                .collect::<Vec<f64>>(),
        );
        Capacities {
            of_trips,
            of_levels,
            of_modes,
        }
    }
    pub fn get_trip(&self, trip: &Trip) -> Count {
        self.of_trips[trip.index]
    }
    pub fn get_level(&self, category: &Category, time_bin: TimeBin) -> Count {
        self.of_levels[category.index][time_bin.value()]
    }
    pub fn get_mode(&self, mode: &Mode) -> Count {
        self.of_modes[mode.index]
    }
    pub fn reduce_trip(&mut self, trip: &Trip, count: Count) {
        assert!(count <= self.of_trips[trip.index]);
        self.of_trips[trip.index] -= count;
    }
    pub fn reduce_level(&mut self, category: &Category, time_bin: TimeBin, count: Count) {
        assert!(count <= self.of_levels[category.index][time_bin.value()]);
        self.of_levels[category.index][time_bin.value()] -= count;
    }
    pub fn reduce_mode(&mut self, mode: &Mode, count: Count) {
        assert!(count <= self.of_modes[mode.index]);
        self.of_modes[mode.index] -= count;
    }
}

fn sum_safe_round(values: &[f64]) -> Vec<usize> {
    let sum: usize = values.iter().sum::<f64>().round() as usize;
    let mut round_values: Vec<usize> = values.iter().map(|x| x.floor() as usize).collect();
    let mut enumerated_diff: Vec<(usize, f64)> = values
        .iter()
        .enumerate()
        .map(|(i, x)| (i, x - (round_values[i] as f64)))
        .collect();
    enumerated_diff.sort_by(|(_, x), (_, y)| y.partial_cmp(x).unwrap()); // desc
    let sum_diff: usize = sum - round_values.iter().sum::<usize>();
    enumerated_diff
        .iter()
        .take(sum_diff)
        .for_each(|&(i, _)| round_values[i] += 1);
    assert!(round_values.iter().sum::<usize>() == sum);
    round_values
}
