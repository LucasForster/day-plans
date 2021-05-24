use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;


fn main() {
    for district in load_districts().iter() {
        println!("{:?}", district);
    }
}

const DISTRICT_COUNT: usize = 647;

#[derive(Debug)]
struct District {
    id: u16,
}

fn load_districts() -> [District; DISTRICT_COUNT] {
    let path = Path::new("verkehrsfluss/verkehrsfluss-zusatz/qz-gebiet-nl.dat");
    let data = read_ascii_file(&path);
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
    districts.try_into().unwrap()
}

fn read_ascii_file(path: &Path) -> String {
    let mut file = File::open(&path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    String::from_utf8_lossy(&data).to_string()
}
