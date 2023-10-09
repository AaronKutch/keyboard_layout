use std::{fs, path::PathBuf};

const DIR: &str = "../rust";
const OUT: &str = "./text.txt";

fn main() {
    let mut all = String::new();
    // Rust 1.73.0 stable branch
    for entry in walkdir::WalkDir::new(DIR) {
        let tmp = entry.unwrap();
        let path = tmp.path().as_os_str().to_str().unwrap();
        if !(path.contains("\\target")
            || path.contains("\\.git")
            || path.contains("/target")
            || path.contains("/.git")
            || path.contains("tests"))
        {
            println!("{path}");
            let path = PathBuf::from(path.to_owned());
            if path.is_file() {
                match fs::read_to_string(&path) {
                    Ok(s) => {
                        all.push_str(&s);
                    }
                    Err(e) => {
                        dbg!(e);
                    }
                }
            }
        }
    }
    fs::write(PathBuf::from(OUT.to_owned()), all).unwrap();
}
