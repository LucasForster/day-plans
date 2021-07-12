use std::collections::HashMap;

use crate::purposes::Purpose;
use crate::trips::TRIPS;

pub fn main() {
    let mut counts: HashMap<(Purpose, Purpose), usize> = HashMap::new();
    for trip in TRIPS.iter() {
        let key = (trip.category.origin, trip.category.destination);
        let count = *counts.get(&key).unwrap_or(&0);
        counts.insert(key, count + trip.count);
    }
    let count_sum: usize = counts.values().sum();
    for (origin, destination) in counts.keys() {
        let count = *counts.get(&(*origin, *destination)).unwrap();
        let percentage = (count as f64) / (count_sum as f64) * 100f64;
        println!("{:?} [{:.1}] {:?}'", origin, percentage, destination);
    }
}
