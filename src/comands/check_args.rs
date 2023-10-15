

pub fn check_args(argumts :  &Vec<String>)-> i32{
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
            3
        },
        4 => {
            eprintln!("Checking modifed files");
            4
        },
        _ => {
            eprintln!("Too many arguments provided");
            std::process::exit(1);
        }
    }
}