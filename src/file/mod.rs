mod file;

use std::fs;
use std::fs::ReadDir;
use std::fs::DirEntry;
use std::borrow::ToOwned;
use std::io::Error;

pub fn hello() -> String {
    "Hello!".to_string()
}

pub fn get_file_names_in_directory(path : &str) -> Vec<String> {
    let mut paths:ReadDir = (fs::read_dir(path)).unwrap();
    let mut file_names:Vec<String> = Vec::new();
    for path in paths {
        //println!("Name: {}", path.unwrap().path().display());
        let y = path.unwrap().path();
        let u = y.to_owned();
        let n = u.to_str();
        file_names.push(n.unwrap().to_owned());
    }
    file_names
}

// use ? instead of try and see if we can condense the method further. 
pub fn get_file_names_in_directory2(path : &str) -> Result<Vec<String>, Error> {
    let mut directory_contents:ReadDir = try!(fs::read_dir(path));
    let mut file_names:Vec<String> = Vec::new();
    for directory_content:Result<DirEntry> in directory_contents {
        let y = try!(directory_content).path();
        let u = y.to_owned();
        let n = u.to_str();
        //file_names.push(try!(path).path().to_str().unwrap);
    }
    Ok(file_names)
}
