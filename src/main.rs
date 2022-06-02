use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs;

const FILE_NAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct HistoryDataFormat {
    type_date: String,
    tag: String,
    text: String,
    health: i32,
    options: Vec<HistoryDataFormat>,
}

impl HistoryDataFormat {
    fn new(row: StringRecord) -> HistoryDataFormat {
        let health = row.get(3).unwrap().parse::<i32>().unwrap_or(0);

        HistoryDataFormat {
            type_date: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            health: health,
            options: Vec::new(),
        }
    }
}

fn main() {
    let mut health = 100;
    let mut current_tag = FIRST_TAG.to_string();

    let mut last_record: String = String::new();

    let mut history_data: HashMap<String, HistoryDataFormat> = HashMap::new();
    let contents = fs::read_to_string(FILE_NAME).unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(contents.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let data = HistoryDataFormat::new(result);

        if data.type_date == "SITUACION" {
            let record_tag = data.tag.clone();

            history_data.insert(record_tag.clone(), data);
            last_record = record_tag;
        } else if data.type_date == "OPCION" {
            if let Some(data_option) = history_data.get_mut(&last_record) {
                (*data_option).options.push(data);
            }
        }
    }

    println!("{:?}", history_data);
}
