use lazy_static::lazy_static;

use std::collections::HashMap;
use std::str::FromStr;

use super::io;
use super::purposes::Purpose;


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Id(u8);
impl Id {
    pub fn value(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Category {
    pub id: Id,
    pub origin: Purpose,
    pub destination: Purpose,
}

lazy_static! {
    pub static ref ID_MAP: HashMap<Id, Category> = load();
}

pub fn parse_id(id: u8) -> Option<Id> {
    if ID_MAP.contains_key(&Id(id)) {
        Some(Id(id))
    } else {
        None
    }
}

fn load() -> HashMap<Id, Category> {
    let records = io::read_csv("verkehrsfluss/verkehrsflussdaten/categoryInformation.txt", false, false, b';', None);
    let mut map: HashMap<Id, Category> = HashMap::new();
    for record in records {
        let split = record[2].split("->").collect::<Vec<&str>>();
        let id = Id(record[0].parse().unwrap());
        if map.contains_key(&id) {
            panic!("Duplicate category id!");
        }
        let origin = Purpose::from_str(split[0]).unwrap();
        let destination = Purpose::from_str(split[1]).unwrap();
        map.insert(id, Category { id, origin, destination });
    }
    println!("Loaded {} categories.", map.len());
    map
}
