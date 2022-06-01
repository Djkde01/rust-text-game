use csv::{ReaderBuilder, StringRecord};
use std::fs;

const FILE_NAME: &str = "history.csv";

fn main() {
    let contents = fs::read_to_string(FILE_NAME).unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(contents.as_bytes());

    for result in rdr.records() {
        println!("Texto: {}", result.unwrap().get(2).unwrap());
    }
}
