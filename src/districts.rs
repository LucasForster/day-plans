use std::collections::HashMap;
use std::convert::TryInto;
use std::path::Path;

use super::io;


type Id = usize;
#[derive(Debug)]
pub struct Index(usize);

const COUNT: usize = 647;

#[derive(Debug)]
pub struct District {
    x: f64,
    y: f64,
    pub info: String,
}

#[derive(Debug)]
pub struct Districts {
    districts: [District; COUNT],
    map: HashMap<Id, Index>,
}

pub fn load() -> Districts {
    let path = Path::new("verkehrsfluss/verkehrsfluss-zusatz/qz-gebiet-nl.dat");
    let data = io::read_ascii_file(&path);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(data.as_bytes());
    let mut districts: Vec<District> = Vec::new();
    let mut map: HashMap<Id, Index> = HashMap::new();
    for result in reader.records() {
        let record = result.unwrap();
        map.insert(record[0].parse().unwrap(), Index(districts.len()));
        districts.push(District {
            x: record[1].parse().unwrap(),
            y: record[2].parse().unwrap(),
            // full concat: (&record.as_slice()[record.range(3).unwrap().start..]).to_string()
            info: if record[3].eq(&record[5]) {record[3].to_string()} else {format!("{} ({})", &record[3], &record[5])},
        });
    }
    Districts {
        districts: districts.try_into().unwrap(),
        map,
    }
}
