#![allow(unused_imports)]
mod comands;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let opt: i32 = comands::check_args::check_args(&args);

    let origin_path = &args[1];
    let destination_path = &args[2];

    let paths = vec![origin_path.to_owned(), destination_path.to_owned()];
    comands::check::check_path(&paths);

    let exist = comands::check_log::check_log(&destination_path);
    comands::log::log(&paths); 
   
    if exist {
        comands::copy_all::copy_all(&paths);
    } else {
        match &opt {
            3 => {
                comands::copy_all::copy_all(&paths);
            },
            4 => {
                comands::copy_modify::compy_modify(&paths);
            },
            _ => {
                eprintln!("Error {}", &opt);
                std::process::exit(1);
            }
        }
    }

}