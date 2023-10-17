use std::time::SystemTime;
use std::{fs, time::Duration};
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Utc};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct ArchivoInfo {
    name: String,
    copydate: String,
    timecopy: String,
    originpath: String,
}

pub fn compy_modify(paths: &Vec<String>) {
    let log_path = format!("{}/log.json", &paths[1]);
    let directory_path = &paths[0];

    let mut file = fs::File::open(log_path).expect("Failed to open log file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Failed to read log file");

    let files_info: Vec<ArchivoInfo> = serde_json::from_str(&json_data).expect("Failed to deserialize JSON");
    let entries = fs::read_dir(directory_path).expect("Failed to read directory");

    for entry in entries {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            if let Some(file_info) = files_info.iter().find(|info| info.name == file_name) {
                let modified_time = entry.metadata().unwrap().modified().unwrap();
                let duration = modified_time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                let log_date = NaiveDate::parse_from_str(&file_info.copydate, "%d-%m-%Y").expect("Failed to parse log date");
                let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

                if duration > Duration::from_secs(0) && duration > log_date.signed_duration_since(epoch).to_std().unwrap() {
                    println!("The file {} has been modified since the last time", file_info.name);
                } else {
                    println!("The file {} has not been modified since last time", file_info.name);
                }
            }
        }
    }
}
