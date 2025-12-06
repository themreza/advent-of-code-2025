use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// From https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}