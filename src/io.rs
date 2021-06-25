use csv::{ReaderBuilder, StringRecord};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_csv<S: AsRef<OsStr>>(
    path: S,
    is_ascii: bool,
    has_headers: bool,
    delimiter: u8,
    comment: Option<u8>,
) -> Vec<StringRecord> {
    let data = if is_ascii {
        read_ascii_file(&Path::new(&path))
    } else {
        read_file(&Path::new(&path))
    };
    let mut reader = ReaderBuilder::new()
        .has_headers(has_headers)
        .flexible(true)
        .delimiter(delimiter)
        .comment(comment)
        .from_reader(data.as_bytes());
    reader.records().map(|result| result.unwrap()).collect()
}

fn read_file(path: &Path) -> String {
    let mut file = File::open(&path).unwrap();
    let mut data: String = "".to_string();
    file.read_to_string(&mut data).unwrap();
    data
}

fn read_ascii_file(path: &Path) -> String {
    let mut file = File::open(&path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    String::from_utf8_lossy(&data).to_string()
}
