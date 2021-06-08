use std::collections::HashMap;
use std::convert::TryInto;
use std::slice::Iter;
use std::time::Duration;

use super::io;


const COUNT: usize = 24;


type Id = usize;

macro_rules! hours {($h:expr) => { Duration::new($h*60*60, 0) }}
macro_rules! minutes {($m:expr) => { Duration::new($m*60, 0) }}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Purpose {
    Home,
    Leisure,
    Work,
    School,
    Service,
    Shopping,
    COUNT
}
impl Purpose {
    fn from_string(string: &str) -> Purpose {
        match string {
            "Arbeit" => Purpose::Work,
            "Dienstleistung" => Purpose::Service,
            "Einkaufen" => Purpose::Shopping,
            "Freizeit" => Purpose::Leisure,
            "Grundschule" => Purpose::School,
            "Hörsaal" => Purpose::School,
            "HörsaalHin" => Purpose::School,
            "Hörsaalplatz" => Purpose::School,
            "HörsaalRück" => Purpose::School,
            "Service" => Purpose::Service,
            "Stud.Ziele" => Purpose::School,
            "weiterf.Schule" => Purpose::School,
            "Wohnen" => Purpose::Home,
            unknown => panic!("Unknown purpose string \"{}\"", unknown),
        }
    }
    pub fn duration(&self) -> Duration {
        match self {
            Purpose::Home => hours!(8),
            Purpose::Work => hours!(8),
            Purpose::School => hours!(7),
            Purpose::Leisure => hours!(2),
            Purpose::Shopping => hours!(1),
            Purpose::Service => minutes!(30),
            Purpose::COUNT => panic!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Category {
    pub id: Id,
    pub origin: Purpose,
    pub destination: Purpose,
}

#[derive(Debug)]
pub struct Categories {
    categories: [Category; COUNT],
    map: HashMap<Id, usize>,
}
impl Categories {
    pub fn get(&self, id: Id) -> Option<&Category> {
        match self.map.get(&id) {
            Some(&index) => Some(&self.categories[index]),
            None => None,
        }
    }
    pub fn iter<'c>(&'c self) -> Iter<'c, Category> {
        self.categories.iter()
    }
}


pub fn load() -> Categories {
    let records = io::read_csv("verkehrsfluss/verkehrsflussdaten/categoryInformation.txt", false, false, b';', None);
    let mut categories: Vec<Category> = Vec::new();
    for record in records {
        let split = record[2].split("->").collect::<Vec<&str>>();
        categories.push(Category {
            id: record[0].parse().unwrap(),
            origin: Purpose::from_string(split[0]),
            destination: Purpose::from_string(split[1]),
        });
    }
    let mut map: HashMap<Id, usize> = HashMap::new();
    for i in 0..categories.len() { // TODO: get iter with index from Vec
        map.insert(categories[i].id, i);
    }
    println!("Loaded {} categories.", categories.len());
    Categories {
        categories: categories.try_into().unwrap(),
        map,
    }
}
