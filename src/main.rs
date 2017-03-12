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


fn main() {
    //fix_import_order2();
    println!("{:?}", 0.cmp(&1));
    read_and_fix_temp("/Users/PKhurana/code/Rust-Java-Import-Order/CreativePropertyFilter.java");
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
    let file_names:Vec<String> = file::get_file_names_in_directory3("/Users/PKhurana/opt/m6d/adserv-a1", file_names).unwrap();
    //println!("final paths of all files {:?}", file_names);

    for file_name in file_names {
        read_and_fix(file_name);
        return Ok(2);
    }
    Ok(3)
}

fn read_and_fix(file_name:String) -> Result<(i32), Error> {
        let mut file_contents:File = try!(File::open(file_name));
        println!("file_contents {:?}", file_contents);
        let mut s:String = String::new();
        file_contents.read_to_string(& mut s);
        println!("s {}", s);
        Ok(2)
}

fn read_and_fix_temp(file_name:&str) -> Result<(i32), Error> {
        let mut file_contents:File = try!(File::open(file_name));
        let mut file = BufReader::new(file_contents);
        let mut v:Vec<Point> = Vec::new();
        for line in file.lines() {
            let l:String = line.unwrap();
            if l.starts_with("import") {
                println!("{}", l);
                let c = Point { x:l };
                v.push(c);
            }
        }
        //v.sort_by(Point::total_cmp);
        println!("{:?}", v);    

        v.sort_by(|a,b| a.total_cmp(b));

        println!("{:?}", v);    
        // println!("file_contents {:?}", file_contents);
        // let mut s:String = String::new();
        // file_contents.read_to_string(& mut s);
        // println!("s {}", s);
        Ok(2)
}

use std::cmp::Ordering;
#[derive(Debug)]
struct Point {
    x: String,
}

impl Point {

    fn getList() -> Vec<String>  {
        let mut v:Vec<String> = Vec::new();
        v.push(String::from("import static"));
        v.push(String::from("import java."));
        v.push(String::from("import javax."));
        v.push(String::from("import org."));
        v.push(String::from("import com."));
        v
    }

    fn total_cmp(&self, rhs: &Self) -> Ordering {
        let c = Point::getList();
        let mut i = 0;
        let mut selfi = 5;
        let mut rhsi = 5;
        // let x = *rhs.ge
         for st in &Point::getList() {
             if self.x.starts_with(st) {
                 selfi = i;
             }
             if rhs.x.starts_with(st) {
                 rhsi = i;
             }
            i = i + 1;
         }

         println!("{} {} {} {}",self.x, selfi, rhs.x, rhsi);

         if selfi == rhsi {
             self.x.cmp(&rhs.x)
         } else {
             selfi.cmp(&rhsi)
         }
    }
}


