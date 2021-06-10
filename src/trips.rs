use super::districts::{Id as DistrictId, parse_id as parse_district_id};
use super::io;
use super::categories::{Id as CategoryId, ID_MAP as CATEGORY_ID_MAP};


type TripCount = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(PartialEq, Eq, Hash)]
pub struct Trip {
    pub transport: Transport,
    pub category_id: CategoryId,
    pub origin: DistrictId,
    pub destination: DistrictId,
    pub count: TripCount,
}



pub fn load<'c>() -> Vec<Trip> {
    let mut trips: Vec<Trip> = Vec::new();
    for transport in vec![Transport::Individual, Transport::Public] {
        for &category_id in CATEGORY_ID_MAP.keys() {
            let path = format!("verkehrsfluss/verkehrsflussdaten/{} ascii.{:03}", transport.to_str(), category_id.value());
            let records = io::read_csv(path, true, false, b' ', Some(b'C'));
            for record in records {
                let count = record[2].parse::<f64>().unwrap().round() as TripCount;
                if count == 0 {
                    continue;
                }
                trips.push(Trip {
                    transport,
                    category_id,
                    origin: parse_district_id(record[0].parse().unwrap()).unwrap(),
                    destination: parse_district_id(record[1].parse().unwrap()).unwrap(),
                    count,
                });
            }
        }
    }
    println!("Loaded {} distinct trips, {} total count.", trips.len(), trips.iter().map(|t| t.count).sum::<TripCount>());
    trips
}
