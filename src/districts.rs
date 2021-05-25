use std::convert::TryInto;
use std::path::Path;

use super::io;

const COUNT: usize = 647;

#[derive(Debug)]
pub struct District {
    id: u16,
}

pub struct Districts {
    districts: [District; COUNT],
}

pub fn load() -> Districts {
    let path = Path::new("verkehrsfluss/verkehrsfluss-zusatz/qz-gebiet-nl.dat");
    let data = io::read_ascii_file(&path);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(data.as_bytes());
    let mut districts : Vec<District> = Vec::new();
    for result in reader.records() {
        let record = result.unwrap();
        districts.push(District {
            id: record[0].parse().unwrap(),
        });
    }
    Districts {
        districts: districts.try_into().unwrap(),
    }
}
