
pub mod file_contents;

use std::fs;
use std::fs::ReadDir;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::io::Error;

// can ignore files starting with dot. are useless.
pub fn get_file_names_in_directory3(path : &str, mut file_names:Vec<String>) -> Result<Vec<String>, Error> {
    let directory_contents:ReadDir = try!(fs::read_dir(path));
    for directory_content in directory_contents {
        let y:DirEntry = try!(directory_content);
        let x:Metadata = try!(y.metadata());
        if x.is_dir() {
            file_names = try!(get_file_names_in_directory3(y.path().to_str().unwrap(), file_names));
        } else {
            match y.path().extension() {
                None => {
                
                },
                Some(i) => {
                    if i.to_str().unwrap().starts_with("java") {
                    file_names.push(y.path().to_str().unwrap().to_string());
                    }
                },
            }
        }
    }
    Ok(file_names)
}
