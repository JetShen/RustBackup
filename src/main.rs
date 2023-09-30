 // let output = Command::new("cmd")
// .arg("/C")
// .arg("xcopy")
// .args(&paths)
// .output()
// .expect("failed to execute process");
// println!("status: {}", output.status);

#![allow(unused_imports)]
mod comands;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    comands::check_args::check_args(&args);

    let origin_path = &args[1];
    let destination_path = &args[2];

    let paths = vec![origin_path.to_owned(), destination_path.to_owned()];
    comands::check::check_path(&paths);
    comands::log::log(origin_path); 
   

    
    
}