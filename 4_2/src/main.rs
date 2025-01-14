use std::fs;
use std::io;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("input.md")?;
    let size = get_size(&contents);
    let mut array: Vec<Vec<char>> = vec![vec!['0';size];size];
    
    
    let reader = BufReader::new(contents.as_bytes());
    let mut line_number = 0;
    for line in reader.lines() {
        array[line_number] = line.unwrap().chars().collect();
        line_number += 1;
    }
    
    let start_points = get_all_start_points(&array);
    
    let base = count_x_mases(&array, &start_points, size, 0);
    let rot90 = count_x_mases(&array, &start_points, size, 1);
    let rot180 = count_x_mases(&array, &start_points, size, 2);
    let rot270 = count_x_mases(&array, &start_points, size, 3);
    
    println!("Count base: `{}`", base);
    println!("Count rot 90: `{}`", rot90);
    println!("Count rot 180: `{}`", rot180);
    println!("Count rot 270: `{}`", rot270);
    
    let total = base + rot90 + rot180 + rot270;
    
    println!("Total: `{}`", total);

    Ok(())
}

fn get_size(contents: &String) -> usize {
    let result = contents.find("\n");
    match result {
        Some(x) => x - 1,
        None => 0
    }
}

fn count_x_mases(array: &Vec<Vec<char>>, starting_points: &Vec<(usize, usize)>, size: usize, rot: usize) -> usize {
    let mut found = 0;
    
    let corners = [(-1, -1), (1, -1), (1, 1), (-1, 1)];
    
    let top_left = corners[(0 + rot)%4];
    let top_right = corners[(1 + rot)%4];
    let bottom_right = corners[(2 + rot)%4];
    let bottom_left = corners[(3 + rot)%4];
    
    for (y, x) in starting_points {
        if (*y as i32 + top_left.1) < 0 || (*y as i32 + top_left.1) >= size as i32 ||
           (*x as i32 + top_left.0) < 0 || (*x as i32 + top_left.0) >= size as i32 {
            continue;
        }
        if (*y as i32 + top_right.1) < 0 || (*y as i32 + top_right.1) >= size as i32 ||
           (*x as i32 + top_right.0) < 0 || (*x as i32 + top_right.0) >= size as i32 {
            continue;
        }
        if (*y as i32 + bottom_right.1) < 0 || (*y as i32 + bottom_right.1) >= size as i32 ||
           (*x as i32 + bottom_right.0) < 0 || (*x as i32 + bottom_right.0) >= size as i32 {
            continue;
        }
        if (*y as i32 + bottom_left.1) < 0 || (*y as i32 + bottom_left.1) >= size as i32 ||
           (*x as i32 + bottom_left.0) < 0 || (*x as i32 + bottom_left.0) >= size as i32 {
            continue;
        }
                             
        if array[(*y as i32 + top_left.1) as usize][(*x as i32 + top_left.0) as usize] != 'M' {
            continue;
        }
        if array[(*y as i32 + top_right.1) as usize][(*x as i32 + top_right.0) as usize] != 'S' {
            continue;
        }
        if array[(*y as i32 + bottom_right.1) as usize][(*x as i32 + bottom_right.0) as usize] != 'S' {
            continue;
        }
        if array[(*y as i32 + bottom_left.1) as usize][(*x as i32 + bottom_left.0) as usize] != 'M' {
            continue;
        }
        
        found += 1;
    }
    
    found
}

fn get_all_start_points(array: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    
    let mut line_index = 0;
    for line in array {
        line.iter().enumerate().filter(|(_, val)| **val == 'A').for_each(|(i, _)| result.push((line_index, i)));        
        line_index += 1;
    }
    result
}