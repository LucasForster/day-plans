use std::sync::Once;

use super::districts::{District, Districts};
use super::io;
use super::purposes::{Category, Categories};


type TripCount = usize;

#[derive(Clone, Copy, Debug)]
pub enum Transport {
    Public,
    Individual,
}
impl Transport {
    const OV: &'static str = "OV";
    const IV: &'static str = "IV";
    const fn to_str(&self) -> &str {
        match *self {
            Transport::Public => Transport::OV,
            Transport::Individual => Transport::IV,
        }
    }
}

#[derive(Debug)]
pub struct Trip<'c, 'd> {
    pub transport: Transport,
    pub category: &'c Category,
    pub origin: &'d District,
    pub destination: &'d District,
    pub count: TripCount,
}

#[derive(Debug)]
pub struct Trips<'c, 'd> {
    pub trips: Vec<Trip<'c, 'd>>,
}


const LOAD: Once = Once::new();
pub fn load<'c, 'd>(categories: &'c Categories, districts: &'d Districts) -> Option<Trips<'c, 'd>> {
    let mut trips = None;
    LOAD.call_once(|| {
        trips = Some(load_file(categories, districts));
    });
    trips
}
fn load_file<'c, 'd>(categories: &'c Categories, districts: &'d Districts) -> Trips<'c, 'd> {
    let mut trips: Vec<Trip> = Vec::new();
    for transport in vec![Transport::Individual, Transport::Public] {
        for category in categories.iter() {
            let path = format!("verkehrsfluss/verkehrsflussdaten/{} ascii.{:03}", transport.to_str(), category.id);
            let records = io::read_csv(path, true, false, b' ', Some(b'C'));
            for record in records {
                let count = record[2].parse::<f64>().unwrap().round() as TripCount;
                if count == 0 {
                    continue;
                }
                trips.push(Trip {
                    transport,
                    category,
                    origin: districts.get(record[0].parse().unwrap()).unwrap(),
                    destination: districts.get(record[1].parse().unwrap()).unwrap(),
                    count,
                });
            }
        }
    }
    Trips {
        trips
    }
}
