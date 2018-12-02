extern crate advent;

use advent::*;
use std::collections::HashSet;

fn main() -> std::result::Result<(), Error> {
    let mut counter: i64 = 0;
    let mut num = HashSet::new();;
    let input = file("input.txt")?;
    let frequency_modifier: std::vec::Vec<&str> = input.split_whitespace().collect();
    let mut broken = false;
    let mut frequency : i64 = 0;

    while !broken {
        for freq in &frequency_modifier {
            let mut freq_adj: i64 = freq.parse()?;
            frequency += freq_adj;
            if num.contains(&frequency) {
                println!("{}", frequency);
                broken = true;
                break;
            } else {
                num.insert(frequency);
            }
        }
        counter += 1;
        println!("Loop: {}", counter);
    }

    Ok(())
}
