use std::cmp::Ordering;
#[derive(Debug)]
pub struct LineOfCode {
    pub line: String,
}

impl LineOfCode {

    pub fn get_list() -> Vec<String>  {
        let mut v:Vec<String> = Vec::new();
        v.push(String::from("import static"));
        v.push(String::from("import java."));
        v.push(String::from("import javax."));
        v.push(String::from("import org."));
        v.push(String::from("import com."));
        v
    }

    pub fn total_cmp(&self, rhs: &Self) -> Ordering {
        let mut i = 0;
        let mut selfi = 5;
        let mut rhsi = 5;
        for st in &LineOfCode::get_list() {
            if self.line.starts_with(st) {
                selfi = i;
            }
            if rhs.line.starts_with(st) {
                rhsi = i;
            }
            i = i + 1;
        }
        if selfi == rhsi {
            if self.line.starts_with(&rhs.line[..rhs.line.len()-1]) {
                Ordering::Greater
            } else  if rhs.line.starts_with(&self.line[..self.line.len()-1]) {
                Ordering::Less
            } else {
                self.line.cmp(&rhs.line)
            }
        } else {
            selfi.cmp(&rhsi)
        }
    }
}