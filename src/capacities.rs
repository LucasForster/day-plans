use super::{
    categories::Category, categories::CATEGORIES,
    levels,
    modes::Mode, modes::MODES,
    time_bins, time_bins::TimeBin, time_bins::TIME_BINS,
    trips::Trip, trips::TRIPS,
};


type Count = usize;

pub struct Capacities {
    of_trips: Vec<Count>,
    of_levels: Vec<[Count; time_bins::COUNT]>,
    of_modes: Vec<Count>,
}
impl Capacities {
    pub fn new() -> Capacities {
        let mut of_levels: Vec<[Count; time_bins::COUNT]> = Vec::new();
        {
            for category in CATEGORIES.iter() {
                assert!(category.index == of_levels.len());
                let mut generator: Generator<TimeBin> = Generator::new(
                    TIME_BINS.into_iter().copied().zip(levels::get_levels(category).to_vec().into_iter()).collect());
                let mut counts = [0 as Count; time_bins::COUNT];
                let total_trip_count: usize = TRIPS.iter().filter(|&trip| trip.category.eq(category)).map(|trip| trip.count).sum();
                for _ in 0..total_trip_count {
                    counts[generator.next().unwrap().value()] += 1;
                }
            }
        }
        let mut of_modes: Vec<Count> = vec![0; MODES.len()];
        {
            let input: Vec<(&Mode, Share)> = MODES.iter().map(|mode| (mode, mode.share)).collect();
            let mut generator = Generator::new(input);
            for mode in generator.take(TRIPS.len()) {
                of_modes[mode.index] += 1;
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
