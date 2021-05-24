use std::fs::File;
use std::io::Read;


fn main() {
    print!("{}", read_ascii_file("verkehrsfluss/verkehrsfluss-zusatz/qz-gebiet-nl.dat".to_string()));
}

fn read_ascii_file(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    unsafe {
        return std::str::from_utf8_unchecked(&data).to_string();
    }
}
