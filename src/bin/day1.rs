extern crate advent;

use advent::*;

fn main() -> std::result::Result<(), Error> {
    let mut frequency : i64= 0;
    let input = file("input.txt")?;
    let frequency_modifier: std::vec::Vec<&str> = input.split_whitespace().collect();

    for freq in frequency_modifier {
        let mut freq_adj: i64 = freq.parse()?;
         frequency += freq_adj
    }

    println!("{:#?}", frequency);
    Ok(())
}

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn file(filename: &str) -> std::result::Result<std::string::String, Error> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

