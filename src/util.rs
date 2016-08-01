use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use super::Result;

pub fn read_path(raw_path: &str) -> Result<String> {

    let path = Path::new(raw_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            error!("couldn't open {}: {}", display, why.description());
            return Err(box why);
        }
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            error!("couldn't read {}: {}", display, why.description());
            return Err(box why);
        }
        Ok(s) => (),
    }
    Ok(s.trim().to_string())
}
