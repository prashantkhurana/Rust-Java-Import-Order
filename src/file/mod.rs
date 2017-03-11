mod file;

use std::fs;
use std::fs::ReadDir;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::borrow::ToOwned;
use std::io::Error;
use std::ffi::OsStr;

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
pub fn get_file_names_in_directory2(path : &str) -> Result<Vec<DirEntry>, Error> {
    let mut directory_contents:ReadDir = try!(fs::read_dir(path));
    let mut file_names:Vec<DirEntry> = Vec::new();
    for directory_content in directory_contents {
        let y = try!(directory_content);
        //let u = y.to_owned();
        //let n = u.to_str();
        file_names.push(y);
    }
    //let c:DirEntry = x;
    //c.
    Ok(file_names)
}

// use ? instead of try and see if we can condense the method further. 
// can ignore files starting with dot. are useless. 
pub fn get_file_names_in_directory3(path : &str, mut file_names:Vec<String>) -> Result<Vec<String>, Error> {
    let mut directory_contents:ReadDir = try!(fs::read_dir(path));
    for directory_content in directory_contents {
        let y:DirEntry = try!(directory_content);
        let x:Metadata = try!(y.metadata());
        if x.is_dir() {
            //println!("is dir");
            //println!("{:?} ",y.path());
            file_names = try!(get_file_names_in_directory3(y.path().to_str().unwrap(), file_names));
        } else {
            // println!("none");
            // println!("{:?} ",y.path());
            match y.path().extension() {
                None => {
                
                },
                Some(i) => {
                    //println!("ddddddd {:?}", i.to_str().unwrap());
                    if i.to_str().unwrap().starts_with("java") {
                    file_names.push(y.path().to_str().unwrap().to_string());
                    }
                },
            }
            //file_names.push(y.path().to_str().unwrap().to_string());
        }
    }
    let x:&str= "dd";
    Ok(file_names)
}
