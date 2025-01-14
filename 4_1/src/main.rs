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
    
    let east = count_direction(&array, &start_points, size, 1, 0);
    let west = count_direction(&array, &start_points, size, -1, 0);
    let north = count_direction(&array, &start_points, size, 0, -1);
    let south = count_direction(&array, &start_points, size, 0, 1);
    let south_east = count_direction(&array, &start_points, size, 1, 1);
    let south_west = count_direction(&array, &start_points, size, -1, 1);
    let north_west = count_direction(&array, &start_points, size, -1, -1);
    let north_east = count_direction(&array, &start_points, size, 1, -1);

    println!("Count east (dir): `{}`", east);
    println!("Count west (dir): `{}`", west);
    println!("Count north (dir): `{}`", north);
    println!("Count south (dir): `{}`", south);
    println!("Count south-east (dir): `{}`", south_east);
    println!("Count south-west (dir): `{}`", south_west);
    println!("Count north-west (dir): `{}`", north_west);
    println!("Count north-east (dir): `{}`", north_east);

    let total = east + west + north + south + south_east + south_west + north_west + north_east;
    
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

fn count_direction(array: &Vec<Vec<char>>, starting_points: &Vec<(usize, usize)>, size: usize, dir_x: i32, dir_y: i32) -> usize {
    let mut found = 0;
    
    for (y, x) in starting_points {
        if (*x as i32 + 1 * dir_x) < 0 || 
           (*x as i32 + 1 * dir_x) >= size as i32 || 
           (*y as i32 + 1 * dir_y) < 0 || 
           (*y as i32 + 1 * dir_y) >= size as i32 || 
           array[(*y as i32 + 1 * dir_y) as usize][(*x as i32 + 1 * dir_x) as usize] != 'M' {
            continue;
        }
        
        if (*x as i32 + 2 * dir_x) < 0 || 
           (*x as i32 + 2 * dir_x) >= size as i32 || 
           (*y as i32 + 2 * dir_y) < 0 || 
           (*y as i32 + 2 * dir_y) >= size as i32 || 
           array[(*y as i32 + 2 * dir_y) as usize][(*x as i32 + 2 * dir_x) as usize] != 'A' {
            continue;
        }
        
        if (*x as i32 + 3 * dir_x) < 0 || 
           (*x as i32 + 3 * dir_x) >= size as i32 || 
           (*y as i32 + 3 * dir_y) < 0 || 
           (*y as i32 + 3 * dir_y) >= size as i32 || 
           array[(*y as i32 + 3 * dir_y) as usize][(*x as i32 + 3 * dir_x) as usize] != 'S' {
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
        line.iter().enumerate().filter(|(_, val)| **val == 'X').for_each(|(i, _)| result.push((line_index, i)));        
        line_index += 1;
    }
    result
}