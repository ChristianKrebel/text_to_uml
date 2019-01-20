use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::io;
use std::vec::Vec;
use std::str::*;
use std::process::exit;

pub fn read_from_file(filename: &str) -> Result<Vec<String>, io::Error> {
    use std::fs;
    let contents = fs::read_to_string(filename)?;
    Ok(contents.lines().map(|line| line.to_string()).collect())
}

pub fn read_from_text(text: &str) -> Result<Vec<String>, io::Error> {
    Ok(text.lines().map(|line| line.to_string()).collect())
}