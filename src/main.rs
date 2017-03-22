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


fn main() {
    match fix_import_order2() {
        Ok(v) => print!("okay"),
        Err(s) => print!("error"),
    };
    //println!("{:?}", 0.cmp(&1));
    //read_and_fix_temp2("/Users/PKhurana/opt/m6d/adserv-a1/adserv-serviceLayer/src/main/java/com/dstillery/duffman/converter/BidSummaryConverter.java");
    //read_and_fix_temp2("/Users/PKhurana/opt/m6d/adserv-a1/adserv-dataLayer/src/main/java/com/dstillery/dao/campaign/CampaignActionTrackerDao.java");
    //read_and_fix_temp2("/Users/PKhurana/opt/m6d/adserv-a1/adserv-dataLayer/src/main/java/com/media6/dao/AgencyDao.java");
    //read_and_fix_temp2("/Users/PKhurana/opt/m6d/adserv-a1/adserv-dataLayer/src/test/java/com/media6/adserv/datalayer/hibernate/JacksonJsonTypeTest.java");



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
    //let file_names:Vec<String> = file::get_file_names_in_directory3("/Users/PKhurana/opt/m6d/adserv-a1/adserv-serviceLayer/src/main/java/com/dstillery/duffman/converter", file_names).unwrap();
    println!("final paths of all files {:?}", file_names);

    for file_name in file_names {
        read_and_fix_temp2(&file_name);
        //return Ok(2);
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
            println!("{}", l);
            if l.starts_with("import") {
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

fn read_and_fix_temp2(file_name:&str) -> Result<(i32), Error> {

        let mut file_contents:File = try!(File::open(file_name));
        let mut contents = String::new();
        file_contents.read_to_string(&mut contents)?;
        let mut ends_with_n = false;
        if contents.ends_with("\n")  {
            ends_with_n = true;
        }
        let mut file_contents:File = try!(File::open(file_name));


        // let mut buffer = [0; 1];
        // file_contents.seek(SeekFrom::End(-1))?;
        // file_contents.read(&mut buffer)?;
        // println!("dddd{:?}", buffer);

        let mut file = BufReader::new(file_contents);
        let mut v:Vec<Point> = Vec::new();
        let mut b:Vec<String> = Vec::new();
        let mut a:Vec<String> = Vec::new();

        let mut file_contents2:File = try!(File::create("/Users/PKhurana/code/Rust-Java-Import-Order/test.java"));
        //fs::copy("foo.txt", "bar.txt")?;  // Copy foo.txt to bar.txt
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;
        for (num, line) in file.lines().enumerate() {
            let mut l:String = line.unwrap();
            //file_contents2.write_all(l.as_bytes());
            //file_contents2.write(b"\n").unwrap();
            //writeln!(file_contents2, l.as_bytes());
            //println!("{}", num);
            if l.starts_with("import") {
                let c = Point { x:l };
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
        v.sort_by(Point::total_cmp);
        let v = add_new_line(v);

        //let mut file = BufReader::new(file_contents);
        //file_contents2.write_all(b);
        // for line2 in b {
        //     //let mut l:String = line2;
        //     file_contents2.write_all(line2.as_bytes());
        //     file_contents2.write(b"\n").unwrap();
        //     //println!("{}", line2);
        // }

        for (i,line2) in b.iter().enumerate() {
            //let mut l:String = line2;
            file_contents2.write_all(line2.as_bytes());
            if i == b.len() -1 {
                print!("not writing");
            } else {
            file_contents2.write(b"\n").unwrap();   
            }
            //println!("{}", line2);
        }
        if v.len() != 0 {
            file_contents2.write(b"\n").unwrap();   
        }




         for line2 in v {
            let mut l:String = line2.x;
            if l == "\n" {
                //print!("new line found");
            }
             file_contents2.write_all(l.as_bytes());
             if l != "\n" {
             file_contents2.write(b"\n").unwrap();   
            }
             //println!("{}", line2);
         }
        file_contents2.write(b"\n").unwrap();   

        println!("ends with {}",ends_with_n);
        for (i,line2) in a.iter().enumerate() {
            //let mut l:String = line2;
            file_contents2.write_all(line2.as_bytes());
            if i == a.len() -1 && ends_with_n == false {
                print!("not writing");
            } else {
            file_contents2.write(b"\n").unwrap();   
            }
            //println!("{}", line2);
        }


        //println!("{:?}", v);    

        //v.sort_by(|a,b| a.total_cmp(b));

        //println!("{:?}", v);  
        println!("file_contents {:?}", file_contents2);
        println!("file_contents {:?}", file_name);

        // let mut s:String = String::new();
        // file_contents.read_to_string(& mut s);
        // println!("s {}", s);
        try!(fs::rename("/Users/PKhurana/code/Rust-Java-Import-Order/test.java", file_name));
        Ok(2)
}

fn add_new_line(v1 : Vec<Point>) -> Vec<Point>{

    let mut old :i32 = -1;
    let mut new :i32 = -1;
    let mut v:Vec<Point> = Vec::new();


    for line2 in v1 {
            new = getI(&line2);
            if old != new && old != -1 {
               let c: Point = Point{x :String::from("\n")};
               v.push(c);
            }
            v.push(line2);
            old = new;

            // if l == "\n" {
            //     print!("new line found");
            // }

            //  file_contents2.write_all(l.as_bytes());
            //  file_contents2.write(b"\n").unwrap();   
             //println!("{}", line2);
         }
    v
}

fn getI(point : &Point) -> i32 {
    let mut selfi:i32 = 5;
    let mut i:i32 = 0;

    for st in &Point::getList() {
             if point.x.starts_with(st) {
                 selfi = i;
             }
            i = i + 1;
         }
         selfi
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

         //println!("{} {} {} {}",self.x, selfi, rhs.x, rhsi);

         if selfi == rhsi {
             if self.x.starts_with(&rhs.x[..rhs.x.len()-1]) {
                 Ordering::Greater
             } else  if rhs.x.starts_with(&self.x[..self.x.len()-1]) {
                 Ordering::Less
             } else {
                self.x.cmp(&rhs.x)
             }
         } else {
             selfi.cmp(&rhsi)
         }
    }
}


