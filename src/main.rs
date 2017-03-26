mod file;

use std::fs;
use std::io::Error;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use file::file_contents::LineOfCode;
use std::io::Write;
use std::env::args;
use std::process;


fn main() {
    let folder_path;
    match args().nth(1)  {
        Some(x) => folder_path = x ,
        None => {
            print!("Please pass the path of the folder to run on as an argument");
            process::exit(1);
        }
    }

    println!("running over  {}",folder_path);
    match fix_import_order(&folder_path) {
        Ok(_) => print!("works!"),
        Err(s) => print!("error {}", s)
    };
    // for running for an individual file
    //read_and_fix_temp2("/Users/PKhurana/opt/m6d/adserv-a1/adserv-serviceLayer/src/main/java/com/dstillery/duffman/converter/BidSummaryConverter.java");
}

fn fix_import_order(folder_path : &str) -> Result<(i32), Error> {
    let file_names:Vec<String> = Vec::new();
    let file_names:Vec<String> = file::get_file_names_in_directory3(folder_path, file_names).unwrap();
    println!("final paths of all files {:?}", file_names);

    for file_name in file_names {
        try!(read_file_and_fix_import(&file_name));
    }
    Ok(1)
}


fn read_file_and_fix_import(file_name:&str) -> Result<(i32), Error> {

        let mut file_contents:File = try!(File::open(file_name));
        let mut contents = String::new();
        file_contents.read_to_string(&mut contents)?;
        let mut ends_with_n = false;
        if contents.ends_with("\n")  {
            ends_with_n = true;
        }
        let file_contents:File = try!(File::open(file_name));

        let file = BufReader::new(file_contents);
        let mut v:Vec<LineOfCode> = Vec::new();
        let mut b:Vec<String> = Vec::new();
        let mut a:Vec<String> = Vec::new();

        let mut file_contents2:File = try!(File::create("/tmp/RustImportThrowaway.java"));

        for line in file.lines() {
            let l:String = line.unwrap();
            if l.starts_with("import") {
                let c = LineOfCode { line:l };
                v.push(c);
            } else if v.len() == 0 {
                b.push(l);
            } else if l.is_empty() && a.len() == 0 {
                //println!("ignore");
            } else {
                a.push(l);
            }
        }

        println!("a is {:?}", a);    
        v.sort_by(LineOfCode::total_cmp);
        let v = add_new_line(v);


        for (i,line2) in b.iter().enumerate() {
            file_contents2.write_all(line2.as_bytes())?;
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
            let l:String = line2.line;
            if l == "\n" {
                //print!("new line found");
            }
             file_contents2.write_all(l.as_bytes())?;
             if l != "\n" {
             file_contents2.write(b"\n").unwrap();   
            }
         }
        file_contents2.write(b"\n").unwrap();   

        println!("ends with {}",ends_with_n);
        for (i,line2) in a.iter().enumerate() {
            file_contents2.write_all(line2.as_bytes())?;
            if i == a.len() -1 && ends_with_n == false {
                print!("not writing");
            } else {
            file_contents2.write(b"\n").unwrap();   
            }
        }

        println!("file_contents {:?}", file_contents2);
        println!("file_contents {:?}", file_name);

        try!(fs::rename("/tmp/RustImportThrowaway.java", file_name));
        Ok(1)
}

fn add_new_line(v1 : Vec<LineOfCode>) -> Vec<LineOfCode>{

    let mut old :i32 = -1;
    let mut new :i32;
    let mut v:Vec<LineOfCode> = Vec::new();


    for line2 in v1 {
            new = get_i(&line2);
            if old != new && old != -1 {
               let c: LineOfCode = LineOfCode { line:String::from("\n")};
               v.push(c);
            }
            v.push(line2);
            old = new;
         }
    v
}

fn get_i(point : &LineOfCode) -> i32 {
    let mut selfi:i32 = 5;
    let mut i:i32 = 0;

    for st in &LineOfCode::get_list() {
             if point.line.starts_with(st) {
                 selfi = i;
             }
            i = i + 1;
         }
         selfi
}