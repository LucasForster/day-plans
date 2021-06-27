use super::categories::{Category, CATEGORIES};
use super::districts::{self, District};
use super::io;
use lazy_static::lazy_static;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub enum Transport {
    Public,
    Individual,
}
impl Transport {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Public => "OV",
            Self::Individual => "IV",
        }
    }
}

pub struct Trip {
    pub index: usize,
    pub transport: Transport,
    pub category: &'static Category,
    pub origin: &'static District,
    pub destination: &'static District,
    pub count: usize,
    _priv: (),
}
impl PartialEq for Trip {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl Eq for Trip {}
impl Hash for Trip {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

lazy_static! {
    pub static ref TRIPS: Vec<Trip> = load();
}
fn load() -> Vec<Trip> {
    let mut trips: Vec<Trip> = Vec::new();
    for transport in vec![Transport::Individual, Transport::Public] {
        for category in CATEGORIES.iter() {
            let path = format!(
                "verkehrsfluss/verkehrsflussdaten/{} ascii.{:03}",
                transport.to_str(),
                category.id.value()
            );
            let records = io::read_csv(path, true, false, b' ', Some(b'C'));
            for record in records {
                let count = record[2].parse::<f64>().unwrap().round() as usize;
                if count == 0 {
                    continue;
                }
                trips.push(Trip {
                    index: trips.len(),
                    transport,
                    category,
                    origin: districts::parse_id(record[0].parse().unwrap()).unwrap(),
                    destination: districts::parse_id(record[1].parse().unwrap()).unwrap(),
                    count,
                    _priv: (),
                });
            }
        }
    }
    println!(
        "Loaded {} distinct trips, {} total count.",
        trips.len(),
        trips.iter().map(|t| t.count).sum::<usize>()
    );
    trips
}
