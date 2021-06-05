use std::collections::HashMap;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use std::sync::Once;

use proj::Proj;

use super::io;


const COUNT: usize = 647;


type Id = usize;

#[derive(Debug)]
pub struct District {
    pub id: Id,
    pub x: f64,
    pub y: f64,
    pub info: String,
}
impl Hash for District {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for District {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for District {}

pub struct Districts {
    districts: [District; COUNT],
    map: HashMap<Id, usize>,
}
impl Districts {
    pub fn get(&self, id: Id) -> Option<&District> {
        match self.map.get(&id) {
            Some(&index) => Some(&self.districts[index]),
            None => None,
        }
    }
}


const LOAD: Once = Once::new();
pub fn load() -> Option<Districts> {
    let mut districts = None;
    LOAD.call_once(|| {
        districts = Some(load_file())
    });
    districts
}
fn load_file() -> Districts {
    let records = io::read_csv("verkehrsfluss/verkehrsfluss-zusatz/qz-gebiet-nl.dat", true, false, b'\t', None);
    let proj = Proj::new_known_crs("EPSG:31466", "EPSG:4326", None).unwrap();
    let mut districts: Vec<District> = Vec::new();
    for record in records {
        let (x, y) = proj.convert((record[1].parse().unwrap(), record[2].parse().unwrap())).unwrap();
        districts.push(District {
            id: record[0].parse().unwrap(),
            x,
            y,
            // full concat: (&record.as_slice()[record.range(3).unwrap().start..]).to_string()
            info: if record[3].eq(&record[5]) {record[3].to_string()} else {format!("{} ({})", &record[3], &record[5])},
        });
    }
    let mut map: HashMap<Id, usize> = HashMap::new();
    for i in 0..districts.len() { // TODO: get iter with index from Vec
        map.insert(districts[i].id, i);
    }
    println!("Loaded {} districts.", districts.len());
    Districts {
        districts: districts.try_into().unwrap(),
        map,
    }
}
