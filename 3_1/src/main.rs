use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

fn main() -> io::Result<()> {
    let file = File::open("input.md")?;
    let mut reader = BufReader::new(file);

    let mut sum = 0;

    loop {
        let bytes_read = reader.skip_until(b'm')?;
        if bytes_read == 0 {
            println!("Finished!");
            break;
        }
        
        if !read_byte(&mut reader, b'u') {
            continue;
        }
        
        if !read_byte(&mut reader, b'l') {
            continue;
        }
        
        if !read_byte(&mut reader, b'(') {
            continue;
        }
        
        let result = read_parameters(&mut reader);
        
        if let Err(_) = result {
            continue;
        }
        
        if let Ok((a, b)) = result {
            sum = sum + (a * b);
        }
    }
 
    println!("Sum: {}", sum);
 
    Ok(())
}

fn read_byte(reader: &mut impl BufRead, expected: u8) -> bool {
    let mut buf = [0;1];
    let result = reader.read(&mut buf);
    
    if result.is_err() {
        return false;
    }
    
    if let Ok(_bytes_read @ 0) = result {
        return false;
    }
    
    buf[0] == expected
}

fn read_parameters(reader: &mut impl BufRead) -> Result<(u32,u32), u32> {
    let mut num1: Vec<u8> = vec![];
    let mut num2: Vec<u8> = vec![];
    
    for _i in 1..=4 {
        let mut buf = [0;1];
        let result = reader.read(&mut buf);
        if result.is_err() {
            return Err(0);
        }
        
        if let Ok(bytes_read) = result {
            if bytes_read == 0 {
                return Err(0);
            }
            match buf[0] {
                b'0'..=b'9' => num1.push(buf[0]),
                b',' => break,
                _ => return Err(0),
            }
        }
    }
    
    if num1.len() == 0 {
        return Err(0);
    }
    
    let num1 = str::from_utf8(&num1).unwrap();
    
    for _i in 1..=4 {
        let mut buf = [0;1];
        let result = reader.read(&mut buf);
        if result.is_err() {
            return Err(0);
        }
        
        if let Ok(bytes_read) = result {
            if bytes_read == 0 {
                return Err(0);
            }
            match buf[0] {
                b'0'..=b'9' => num2.push(buf[0]),
                b')' => break,
                _ => return Err(0),
            }
        }
    }
    
    if num2.len() == 0 {
        return Err(0);
    }
    
    let num2 = str::from_utf8(&num2).unwrap();
        
    return Ok((num1.parse::<u32>().unwrap(),num2.parse::<u32>().unwrap()))
}