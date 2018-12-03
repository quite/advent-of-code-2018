use std::fs::File;
use std::io::prelude::*;

pub fn read_from_file(path: &str) -> std::io::Result<String> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}
