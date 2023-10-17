use std::fs;

//// Checks if the file "log.json" exists in the specified location.
///
/// # Arguments
///
/// * `destination_path` - The destination path where the file will be searched.
///
/// # Returns
///
/// `true` if the file exists, `false` otherwise.
pub fn check_log(destination_path: &str) -> bool {
    let destination = format!("{}/log.json", destination_path);
    println!("Ruta del archivo: {}", &destination);

    match fs::metadata(&destination) {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}