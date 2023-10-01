use serde::{Deserialize, Serialize};

use std::process::Command;
use std::{str, path};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct ArchivoInfo {
    name: String,
    copydate: String,
    timecopy: String,
    originpath: String,
}


pub fn log(paths :  &Vec<String>){
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Get-ChildItem -File -Recurse -Path {} | Select-Object LastWriteTime, Name, FullName | Format-Table -AutoSize",
            &paths[0]
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
                    name: parts[2].to_owned().to_string(),
                    copydate:  parts[0].to_owned(),
                    timecopy: parts[1].to_owned(),
                    originpath: parts[3].to_owned(),
                });
            }
        }

       
        let json_data = serde_json::to_string(&archivos_info).expect("failed to convert in json");

        let mut file = fs::File::create(format!("{}\\log.json", &paths[1])).expect("failed to create file");
        file.write_all(json_data.as_bytes()).expect("filed to write");
        

    } else {
        let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
        eprintln!("{}", stderr);
        std::process::exit(1);
    }
}