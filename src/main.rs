mod file;

use std::fs::DirEntry;
use std::fs;
use std::path::Path;
use std::io::Error;
use std::fs::Metadata;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::vec;
use std::ops::FnMut;
use std::io::Write;
use file::file_contents::line_of_code;


fn main() {
    match fix_import_order2() {
        Ok(v) => print!("okay"),
        Err(s) => print!("error"),
    };
    //read_and_fix_temp2("/Users/PKhurana/opt/m6d/adserv-a1/adserv-serviceLayer/src/main/java/com/dstillery/duffman/converter/BidSummaryConverter.java");
    ()
}

fn fix_import_order2() -> Result<(i32), Error> {
    let mut file_names:Vec<String> = Vec::new();
    let file_names:Vec<String> = file::get_file_names_in_directory3("/Users/PKhurana/opt/m6d/adserv-a1", file_names).unwrap();
    println!("final paths of all files {:?}", file_names);

    for file_name in file_names {
        read_and_fix_temp2(&file_name);
    }
    Ok(3)
}


fn read_and_fix_temp2(file_name:&str) -> Result<(i32), Error> {

        let mut file_contents:File = try!(File::open(file_name));
        let mut contents = String::new();
        file_contents.read_to_string(&mut contents)?;
        let mut ends_with_n = false;
        if contents.ends_with("\n")  {
            ends_with_n = true;
        }
        let mut file_contents:File = try!(File::open(file_name));

        let mut file = BufReader::new(file_contents);
        let mut v:Vec<line_of_code> = Vec::new();
        let mut b:Vec<String> = Vec::new();
        let mut a:Vec<String> = Vec::new();

        let mut file_contents2:File = try!(File::create("/Users/PKhurana/code/Rust-Java-Import-Order/test.java"));
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;
        for (num, line) in file.lines().enumerate() {
            let mut l:String = line.unwrap();
            if l.starts_with("import") {
                let c = line_of_code { line:l };
                v.push(c);
                l = String::from("222");
            } else if v.len() == 0 {
                b.push(l);
            } else if l.is_empty() && a.len() == 0 {
                //println!("ignore");
            } else {
                a.push(l);
            }
        }

        println!("a is {:?}", a);    
        v.sort_by(line_of_code::total_cmp);
        let v = add_new_line(v);


        for (i,line2) in b.iter().enumerate() {
            file_contents2.write_all(line2.as_bytes());
            if i == b.len() -1 {
                print!("not writing");
            } else {
            file_contents2.write(b"\n").unwrap();   
            }
        }
        if v.len() != 0 {
            file_contents2.write(b"\n").unwrap();   
        }




         for line2 in v {
            let mut l:String = line2.line;
            if l == "\n" {
                //print!("new line found");
            }
             file_contents2.write_all(l.as_bytes());
             if l != "\n" {
             file_contents2.write(b"\n").unwrap();   
            }
         }
        file_contents2.write(b"\n").unwrap();   

        println!("ends with {}",ends_with_n);
        for (i,line2) in a.iter().enumerate() {
            file_contents2.write_all(line2.as_bytes());
            if i == a.len() -1 && ends_with_n == false {
                print!("not writing");
            } else {
            file_contents2.write(b"\n").unwrap();   
            }
        }

        println!("file_contents {:?}", file_contents2);
        println!("file_contents {:?}", file_name);

        try!(fs::rename("/Users/PKhurana/code/Rust-Java-Import-Order/test.java", file_name));
        Ok(2)
}

fn add_new_line(v1 : Vec<line_of_code>) -> Vec<line_of_code>{

    let mut old :i32 = -1;
    let mut new :i32 = -1;
    let mut v:Vec<line_of_code> = Vec::new();


    for line2 in v1 {
            new = getI(&line2);
            if old != new && old != -1 {
               let c: line_of_code = line_of_code { line:String::from("\n")};
               v.push(c);
            }
            v.push(line2);
            old = new;
         }
    v
}

fn getI(point : &line_of_code) -> i32 {
    let mut selfi:i32 = 5;
    let mut i:i32 = 0;

    for st in &line_of_code::getList() {
             if point.line.starts_with(st) {
                 selfi = i;
             }
            i = i + 1;
         }
         selfi
}