use serde::{Deserialize, Serialize};

use std::process::Command;
use std::{str, path};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct ArchivoInfo {
    nombre: String,
    fechacopia: String,
    horacopia: String,
    originpath: String,
}


pub fn log(path  : &str){
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Get-ChildItem -File -Recurse -Path {} | Select-Object LastWriteTime, Name, FullName | Format-Table -AutoSize",
            path
        ))
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
        let lines: Vec<&str> = stdout.lines().collect();
        let mut archivos_info: Vec<ArchivoInfo> = Vec::new();

        for line in lines.iter().skip(3) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                archivos_info.push(ArchivoInfo {
                    nombre: parts[2].to_owned().to_string(),
                    fechacopia:  parts[0].to_owned(),
                    horacopia: parts[1].to_owned(),
                    originpath: parts[3].to_owned(),
                });
            }
        }

        // Convertir archivos_info a JSON
        let json_data = serde_json::to_string(&archivos_info).expect("Error al serializar a JSON");

        // Imprimir o guardar el JSON
        println!("{}", json_data);
    } else {
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
        eprintln!("{}", stderr);
        std::process::exit(1);
    }
}