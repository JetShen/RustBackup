use std::process::Command;

pub fn copy_all(paths : &Vec<String>){
    let copy: std::process::Output = Command::new("cmd")
        .arg("/C")
        .arg("xcopy")
        .arg(&paths[0])
        .arg(&paths[1])
        .arg("/E")
        .output()
        .expect("failed to copy");

    if copy.status.success() {
        println!("copia realizada")
    } else {
        println!("Error al copiar {}" , copy.status)
    }
}