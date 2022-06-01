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

impl HistoryDataFormat {
    fn new(row: StringRecord) -> HistoryDataFormat {
        let health = row.get(3).unwrap().parse::<i32>().unwrap_or(0);

        HistoryDataFormat {
            type_date: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            health: health,
        }
    }
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
        let data = HistoryDataFormat::new(result);
        history_data.push(data);
    }

    println!("{:?}", history_data);
}
