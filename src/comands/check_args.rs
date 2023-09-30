

pub fn check_args(argumts :  &Vec<String>){
    match argumts.len() {
        1 => {
            eprintln!("Use: program <origen> <destino>");
            std::process::exit(1);
        },
        2 => {
            eprintln!("Please provide the destination path");
            std::process::exit(1);
        },
        3 => {
        },
        _ => {
            eprintln!("Too many arguments provided");
            std::process::exit(1);
        }
    }
}