use super::{
    io,
    purposes::Purpose,
};
use std::str::FromStr;
use lazy_static::lazy_static;


pub struct Category {
    pub index: usize,
    pub id: u8,
    pub origin: Purpose,
    pub destination: Purpose,
    _priv: (),
}
impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Category {}


lazy_static! {
    pub static ref CATEGORIES: Vec<Category> = load();
}

fn load() -> Vec<Category> {
    let records = io::read_csv("verkehrsfluss/verkehrsflussdaten/categoryInformation.txt", false, false, b';', None);
    let mut categories: Vec<Category> = Vec::new();
    for record in records {
        let split = record[2].split("->").collect::<Vec<&str>>();
        let id: u8 = record[0].parse().unwrap();
        if categories.iter().find(|&category| category.id == id).is_some() {
            panic!("Duplicate category id!");
        }
        let origin = Purpose::from_str(split[0]).unwrap();
        let destination = Purpose::from_str(split[1]).unwrap();
        categories.push(Category { index: categories.len(), id, origin, destination, _priv: () });
    }
    println!("Loaded {} categories.", categories.len());
    categories
}
