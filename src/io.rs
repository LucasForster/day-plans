use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use csv::{ReaderBuilder, StringRecord};


pub fn read_csv<S: AsRef<OsStr>>(path: S, hasHeaders: bool, delimiter: u8) -> Vec<StringRecord> {
    let data = read_ascii_file(&Path::new(&path));
    let mut reader = ReaderBuilder::new()
        .has_headers(hasHeaders)
        .delimiter(delimiter)
        .from_reader(data.as_bytes());
    reader.records().map(|result| result.unwrap()).collect()
}

fn read_ascii_file(path: &Path) -> String {
    let mut file = File::open(&path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    String::from_utf8_lossy(&data).to_string()
}
