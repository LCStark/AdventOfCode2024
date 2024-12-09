use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("input.md")?;
    let reader = BufReader::new(file);
    
    let mut first: Vec<i32> = vec!();
    let mut second: Vec<i32> = vec!();
    
    let mut distances: Vec<i32> = vec!();
    
    for line in reader.lines() {
        let row = line.unwrap();       
        let v: Vec<&str> = row.split("   ").collect();
        
        first.push(v[0].parse::<i32>().unwrap());
        second.push(v[1].parse::<i32>().unwrap());
    }
    
    while first.len() > 0 {
        let min_first = first.iter().min().unwrap();
        let min_second = second.iter().min().unwrap();
        
        distances.push((min_first - min_second).abs());
        
        let found_first = first.iter().position(|x| x == min_first);
        let found_second = second.iter().position(|x| x == min_second);
                
        first.remove(found_first.unwrap());
        second.remove(found_second.unwrap());
    }
    
    let sum: i32 = distances.iter().sum();
    
    println!("The sum of distances: {}", sum);
    
    Ok(())
}
