use std::collections::HashMap;

use lazy_static::lazy_static;

use super::io;


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Id(u16);

pub struct District {
    pub id: Id,
    pub x: f64,
    pub y: f64,
    pub info: String,
}

lazy_static! {
    pub static ref ID_MAP: HashMap<Id, District> = load();
}

pub fn parse_id(id: u16) -> Option<Id> {
    if ID_MAP.contains_key(&Id(id)) {
        Some(Id(id))
    } else {
        None
    }
}

fn load() -> HashMap<Id, District> {
    let records = io::read_csv("verkehrsfluss/verkehrsfluss-zusatz/qz-gebiet-nl.dat", true, false, b'\t', None);
    let proj = proj::Proj::new_known_crs("EPSG:31466", "EPSG:4326", None).unwrap();
    let mut map = HashMap::<Id, District>::new();
    for record in records {
        let id = Id(record[0].parse().unwrap());
        if map.contains_key(&id) {
            panic!("Duplicate district id!");
        }
        let (x, y) = proj.convert((record[1].parse().unwrap(), record[2].parse().unwrap())).unwrap();
        let info = compose_info(&record);
        map.insert(id, District { id, x, y, info });
    }
    println!("Loaded {} districts.", map.len());
    map
}
fn compose_info(record: &csv::StringRecord) -> String {
    if record[3].eq(&record[5]) {
        record[3].to_string()
    } else {
        format!("{} ({})", &record[3], &record[5])
    }
}
