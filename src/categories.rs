use super::io;
use super::purposes::Purpose;
use lazy_static::lazy_static;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(PartialEq, Eq)]
pub struct Id(u8);
impl Id {
    pub fn value(&self) -> u8 {
        self.0
    }
}
pub struct Category {
    pub index: usize,
    pub id: Id,
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
impl Hash for Category {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.0.hash(state);
    }
}

lazy_static! {
    pub static ref CATEGORIES: Vec<Category> = load();
}

fn load() -> Vec<Category> {
    let records = io::read_csv(
        "verkehrsfluss/verkehrsflussdaten/categoryInformation.txt",
        false,
        false,
        b';',
        None,
    );
    let mut categories: Vec<Category> = Vec::new();
    for record in records {
        let split = record[2].split("->").collect::<Vec<&str>>();
        let id: u8 = record[0].parse().unwrap();
        if categories
            .iter()
            .find(|&category| category.id.0 == id)
            .is_some()
        {
            panic!("Duplicate category id!");
        }
        let origin = Purpose::from_str(split[0]).unwrap();
        let destination = Purpose::from_str(split[1]).unwrap();
        categories.push(Category {
            index: categories.len(),
            id: Id(id),
            origin,
            destination,
            _priv: (),
        });
    }
    println!("Loaded {} categories.", categories.len());
    categories
}
