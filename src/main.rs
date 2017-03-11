mod file;

use std::fs::DirEntry;
use std::fs;
use std::path::Path;
use std::io::Error;
use std::fs::Metadata;


fn main() {
    fix_import_order2();
    // println!("test {}" ,file::hello());
    // println!("Hello, world!");
    // let file_names:Vec<DirEntry> = file::get_file_names_in_directory2("/Users/PKhurana/code/Rust-Java-Import-Order").unwrap();
    // println!("test {:?}", file_names);
    ()
}

fn fix_import_order() -> Result<(i32), Error> {
    let file_names:Vec<DirEntry> = file::get_file_names_in_directory2("/Users/PKhurana/code/Rust-Java-Import-Order").unwrap();
    println!("test {:?}", file_names);
    let absolute_file_names:Vec<String> = Vec::new();
    for file_name in file_names {
        let f:DirEntry = file_name;
        let x:Metadata = f.metadata().unwrap();
        if x.is_dir() {

        }
    }

    Ok(3)
}

fn fix_import_order2() -> Result<(i32), Error> {
    let mut file_names:Vec<String> = Vec::new();
    let file_names:Vec<String> = file::get_file_names_in_directory3("/Users/PKhurana/code/Rust-Java-Import-Order", file_names).unwrap();
    println!("final paths of all files {:?}", file_names);
    // let absolute_file_names:Vec<String> = Vec::new();
    // for file_name in file_names {
    //     let f:DirEntry = file_name;
    //     let x:Metadata = f.metadata().unwrap();
    //     if x.is_dir() {
            
    //     }
    // }

    Ok(3)
}


