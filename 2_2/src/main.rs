use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    let file = File::open("input.md")?;
    let reader = BufReader::new(file);
    
    let mut safe_reports = 0;
    
    for line in reader.lines() {
        let row = line.unwrap();
        
        if is_row_safe(&row) {
            safe_reports += 1
        }
    }
    
    println!("Safe reports: {}", safe_reports);
    
    Ok(())
}

fn is_row_safe(line: &String) -> bool {
    let row: Vec<_> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        
    if row[0] == row[1] {
        return false
    }
    
    let increasing = row[1] > row[0];
    
    let mut previous = row[0];
    
    for number in &row[1..] {
        let current = number;
        let diff = current - previous;
        
        if (increasing && diff <=0) || (!increasing && diff >= 0) {
            return false;
        }
        
        if diff.abs() > 3 {
            return false;
        }
        previous = *current;
    }
    
    true
}