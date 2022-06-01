use csv::{ReaderBuilder, StringRecord};
use std::fs;

const FILE_NAME: &str = "history.csv";

#[derive(Debug)]
struct HistoryDataFormat {
    type_date: String,
    tag: String,
    text: String,
    health: i32,
}

fn main() {
    let mut history_data: Vec<HistoryDataFormat> = Vec::new();
    let contents = fs::read_to_string(FILE_NAME).unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(contents.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let data = HistoryDataFormat {
            type_date: result.get(0).unwrap().trim().to_string(),
            tag: result.get(1).unwrap().trim().to_string(),
            text: result.get(2).unwrap().trim().to_string(),
            health: 0,
        };
        history_data.push(data);
    }

    println!("{:?}", history_data);
}
