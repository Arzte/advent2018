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
