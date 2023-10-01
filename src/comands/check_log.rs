use std::fs;

pub fn check_log(destination_path : &String) -> bool {
    let mut first_time = false;
    if let Ok(metadata) = fs::metadata(destination_path) {
        if metadata.is_file() {
            first_time = true;
        }
    } else {
        first_time = true;
    }
    first_time
}