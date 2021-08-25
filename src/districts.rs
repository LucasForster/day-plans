use super::io;
use lazy_static::lazy_static;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Id(u16);

#[derive(Debug)]
pub struct District {
    pub index: usize,
    pub id: Id,
    pub x: f64,
    pub y: f64,
    pub info: String,
    _priv: (),
}
impl PartialEq for District {
    fn eq(&self, other: &Self) -> bool {
        self.id.0 == other.id.0
    }
}
impl Eq for District {}
impl Hash for District {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.0.hash(state);
    }
}

lazy_static! {
    pub static ref DISTRICTS: Vec<District> = load();
}

pub fn parse_id(id: u16) -> Option<&'static District> {
    DISTRICTS.iter().find(|&district| district.id.0 == id)
}

fn load() -> Vec<District> {
    let records = io::read_csv(
        "verkehrsfluss/verkehrsfluss-zusatz/qz-gebiet-nl.dat",
        true,
        false,
        b'\t',
        None,
    );
    let mut vec: Vec<District> = Vec::new();
    for record in records {
        let id = Id(record[0].parse().unwrap());
        assert!(vec.iter().find(|&district| district.id.0 == id.0).is_none());
        let (x, y) = (record[1].parse().unwrap(), record[2].parse().unwrap());
        let info = compose_info(&record);
        vec.push(District {
            index: vec.len(),
            id,
            x,
            y,
            info,
            _priv: (),
        });
    }
    println!("Loaded {} districts.", vec.len());
    vec
}
fn compose_info(record: &csv::StringRecord) -> String {
    if record[3].eq(&record[5]) {
        record[3].to_string()
    } else {
        format!("{} ({})", &record[3], &record[5])
    }
}
