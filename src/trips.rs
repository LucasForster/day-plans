use super::{
    categories::Category, categories::CATEGORIES,
    districts, districts::District,
    io,
};

use lazy_static::lazy_static;


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

lazy_static! {
    pub static ref TRIPS: Vec<Trip> = load();
}
fn load() -> Vec<Trip> {
    let mut trips: Vec<Trip> = Vec::new();
    for transport in vec![Transport::Individual, Transport::Public] {
        for category in CATEGORIES.iter() {
            let path = format!("verkehrsfluss/verkehrsflussdaten/{} ascii.{:03}", transport.to_str(), category.id);
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
    println!("Loaded {} distinct trips, {} total count.", trips.len(), trips.iter().map(|t| t.count).sum::<usize>());
    trips
}
