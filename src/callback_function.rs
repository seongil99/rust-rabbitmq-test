use std::{fs::File, io::Write};

pub fn write_file_all(s: &str) {
    let mut file = File::create("test.txt").unwrap();
    file.write_all(s.as_bytes()).unwrap();
}
