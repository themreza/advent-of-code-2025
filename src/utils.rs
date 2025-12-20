use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::{Path},
};

// From https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file: File = File::open(filename).expect("no such file");
    let buf: BufReader<File> = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[allow(dead_code)]
pub fn str_slice_to_vec_string(input: &[&str]) -> Vec<String> {
    input.iter().map(|s| s.to_string()).collect()
}

pub fn string_to_file(content: &str) -> String {
    let path = std::env::temp_dir().join(format!("tmp_{}", std::process::id()));
    std::fs::write(&path, content).unwrap();
    path.into_os_string().into_string().expect("failed to get temp file path")
}
