use std::collections::HashMap;

use csv::StringRecord;
use proj::Proj;

use super::io;


#[derive(PartialEq, Eq, Hash)]
pub struct Id(usize);

pub struct District {
    pub x: f64,
    pub y: f64,
    pub info: String,
}

pub struct Districts {
    map: HashMap<Id, District>,
}
impl Districts {
    pub fn get(&self, id: &Id) -> &District {
        self.map.get(id).unwrap()
    }
    pub fn id(&self, id: usize) -> Option<&Id> {
        match self.map.get_key_value(&Id(id)) {
            Some(key_value) => Some(key_value.0),
            None => None,
        }
    }
}

pub fn load() -> Districts {
    let records = io::read_csv("verkehrsfluss/verkehrsfluss-zusatz/qz-gebiet-nl.dat", true, false, b'\t', None);
    let proj = Proj::new_known_crs("EPSG:31466", "EPSG:4326", None).unwrap();
    let mut map = HashMap::<Id, District>::new();
    for record in records {
        let id = Id(record[0].parse().unwrap());
        if map.contains_key(&id) {
            panic!("Duplicate district id!");
        }
        let (x, y) = proj.convert((record[1].parse().unwrap(), record[2].parse().unwrap())).unwrap();
        let info = compose_info(&record);
        map.insert(id, District { x, y, info });
    }
    println!("Loaded {} districts.", map.len());
    Districts { map }
}
fn compose_info(record: &StringRecord) -> String {
    if record[3].eq(&record[5]) {
        record[3].to_string()
    } else {
        format!("{} ({})", &record[3], &record[5])
    }
}
