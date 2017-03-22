
pub mod file_contents;

use std::fs;
use std::fs::ReadDir;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::io::Error;

// use ? instead of try and see if we can condense the method further.
// can ignore files starting with dot. are useless. 
pub fn get_file_names_in_directory3(path : &str, mut file_names:Vec<String>) -> Result<Vec<String>, Error> {
    let directory_contents:ReadDir = try!(fs::read_dir(path));
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
    Ok(file_names)
}
