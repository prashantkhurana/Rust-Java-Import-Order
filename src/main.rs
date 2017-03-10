mod file;

use std::fs::DirEntry;
use std::fs;
use std::path::Path;

fn main() {
    println!("test {}" ,file::hello());
    println!("Hello, world!");
    //let file_names:Vec<DirEntry> = file::get_file_names_in_directory2("/Users/PKhurana/code/Rust-Java-Import-Order").unwrap();
    
    ()
}


