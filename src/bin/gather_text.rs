use std::{fs, path::PathBuf};

fn main() {
    let mut all = String::new();
    for entry in walkdir::WalkDir::new("../awint") {
        let tmp = entry.unwrap();
        let path = tmp.path().as_os_str().to_str().unwrap();
        if !(path.contains("\\target") || path.contains("\\.git")) {
            println!("{path}");
            let path = PathBuf::from(path.to_owned());
            if path.is_file() {
                all.push_str(&fs::read_to_string(&path).unwrap());
            }
        }
    }
    // get all ascii lowercase
    let mut all1: Vec<u8> = vec![];
    for c in all.chars() {
        if c.is_ascii() {
            all1.push(u8::try_from(c.to_ascii_lowercase()).unwrap());
        }
    }
    // convert groups of 4 spaces to tabs
    let mut all2 = vec![];
    let mut space_count = 0;
    for c in &all1 {
        all2.push(*c);
        if *c == b' ' {
            space_count += 1;
        } else {
            space_count = 0;
        }
        if space_count == 4 {
            all2.pop();
            all2.pop();
            all2.pop();
            all2.pop();
            all2.push(b'\t');
            space_count = 0;
        }
    }
    fs::write(PathBuf::from("./text.txt".to_owned()), all1).unwrap();
}
