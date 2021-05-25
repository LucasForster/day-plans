use std::collections::HashMap;
use std::convert::TryInto;
use std::path::Path;
use std::sync::Once;

use super::io;


const COUNT: usize = 647;


#[derive(Debug, Hash, PartialEq, Clone, Copy)]
pub struct Id(usize);
impl Eq for Id {}

#[derive(Debug)]
pub struct District {
    pub id: Id,
    pub x: f64,
    pub y: f64,
    pub info: String,
}

#[derive(Debug)]
pub struct Districts {
    districts: [District; COUNT],
    map: HashMap<Id, usize>,
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
    let path = Path::new("verkehrsfluss/verkehrsfluss-zusatz/qz-gebiet-nl.dat");
    let data = io::read_ascii_file(&path);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(data.as_bytes());
    let mut districts: Vec<District> = Vec::new();
    for result in reader.records() {
        let record = result.unwrap();
        districts.push(District {
            id: Id(record[0].parse().unwrap()),
            x: record[1].parse().unwrap(),
            y: record[2].parse().unwrap(),
            // full concat: (&record.as_slice()[record.range(3).unwrap().start..]).to_string()
            info: if record[3].eq(&record[5]) {record[3].to_string()} else {format!("{} ({})", &record[3], &record[5])},
        });
    }
    let mut map: HashMap<Id, usize> = HashMap::new();
    for i in 0..districts.len() { // TODO: get iter with index from Vec
        map.insert(districts[i].id, i);
    }
    Districts {
        districts: districts.try_into().unwrap(),
        map,
    }
}
