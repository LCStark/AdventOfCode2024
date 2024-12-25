use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("input.md")?;
    let reader = BufReader::new(file);
    
    let mut first: Vec<i32> = vec!();
    let mut second: Vec<i32> = vec!();
    
    let mut score: i32 = 0;
    
    for line in reader.lines() {
        let row = line.unwrap();       
        let v: Vec<&str> = row.split("   ").collect();
        
        first.push(v[0].parse::<i32>().unwrap());
        second.push(v[1].parse::<i32>().unwrap());
    }
    
    for number in first {
        let sum: i32 = second.iter().map(|x| if *x == number { 1 } else { 0 }).sum();
        score += number * sum;
    }
    
    println!("Total score: {}", score);
    
    Ok(())
}