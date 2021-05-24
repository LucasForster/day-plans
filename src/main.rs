use std::fs::File;
use std::io::Read;
use std::path::Path;


fn main() {
    print!("{}", read_ascii_file(Path::new("verkehrsfluss/verkehrsfluss-zusatz/qz-gebiet-nl.dat")));
}

fn read_ascii_file(path: &Path) -> String {
    let mut file = File::open(&path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    String::from_utf8_lossy(&data).to_string()
}
