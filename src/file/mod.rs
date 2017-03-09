mod file;

use std::fs;
use std::fs::ReadDir;

pub fn hello() -> String {
    "Hello!".to_string()
}

pub fn get_file_names_in_directory(path : &str) -> Vec<&str> {
    let mut paths:ReadDir = fs::read_dir(path).unwrap();
    let mut file_names:Vec<&str> = Vec::new();
    for path in paths {
        //println!("Name: {}", path.unwrap().path().display());
        let y = path.unwrap().path();
        let u = y.to_owned();
        //let n = u.to_str();
        file_names.push(u.to_str());
    }
    Vec::new()
}
