use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn input(name: &str) -> io::Result<String> {
    let mut s = String::new();
    File::open(Path::new("input").join(name))?.read_to_string(&mut s)?;
    Ok(s)
}
