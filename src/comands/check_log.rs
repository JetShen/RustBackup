use std::fs;

pub fn check_log(destination_path : &String) -> bool {
    let mut first_time = true;
    let destination = format!("{destination_path}\\log.json");
    println!("{}",&destination);
    if let Ok(metadata) = fs::metadata(&destination) {
        if metadata.is_file() { 
            first_time = false;
        }
    } else {
        first_time = true;
    }
    first_time
}