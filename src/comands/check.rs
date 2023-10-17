use std::process::Command;
use std::fs;

pub fn check_path(paths : &Vec<String>){
    for path in paths {
        if !fs::metadata(path).is_ok() {
            eprintln!("Path {} not found", path);
            std::process::exit(1);
        }
    }
}