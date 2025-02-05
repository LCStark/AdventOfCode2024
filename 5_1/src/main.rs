use std::fs;
use std::io;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> io::Result<()> {    
    let file = fs::File::open("input.md")?;
    let mut reader = BufReader::new(file);
    
    let (rules, updates) = read_rules_and_updates(&mut reader);
    
    let before_rules = process_rules(&rules);
    
    let correct_updates:Vec<String> = updates.into_iter().filter(|x| check_update(x, &before_rules)).collect();
    
    let middle_values: Vec<u32> = correct_updates.iter().map(|x| middle_value(&x).parse::<u32>().unwrap()).collect();
    
    let sum: u32 = middle_values.iter().sum();
    
    println!("Sum: {}", sum);
        
    Ok(())
}

fn middle_value(update: &String) -> String {
    let split = update.split(",").collect::<Vec<&str>>();
    let count = split.len();
    let mid = count / 2;
    split[mid].to_string()
}

fn check_update(update: &String, before_rules: &Vec<(String, Vec<String>)>) -> bool {
    
    let split = update.split(",").collect::<Vec<&str>>();
    
    for (index, page) in split.iter().enumerate() {
        
        let page = page.to_string();
        
        let ruleset_index = before_rules.iter().position(|x| x.0 == page);
        if ruleset_index.is_none() {
            continue;
        }
        let ruleset_index = ruleset_index.unwrap();
        
        let rule_set = &before_rules[ruleset_index];
                
        for rule in &rule_set.1 {
            let rule_index = split.iter().position(|x| x == rule);
            if rule_index.is_none() || rule_index.unwrap() > index {
                continue;
            }
            return false;
        }
    }
    
    true
}

fn process_rules(rules: &Vec<String>) -> Vec<(String, Vec<String>)> {
    
    let mut ready_rules: Vec<(String, Vec<String>)> = vec![];
    
    for rule in rules {
        let split = rule.split("|").collect::<Vec<&str>>();
        
        let index = ready_rules.iter().position(|x| x.0 == split[0]);
        if index.is_none() {
            ready_rules.push((split[0].to_string(), vec![split[1].to_string()]));
        } else {
            ready_rules[index.unwrap()].1.push(split[1].to_string());
        }
    }
    
    ready_rules
}


fn read_rules_and_updates(reader: &mut BufReader<fs::File>) -> (Vec<String>, Vec<String>) {
    let mut rules : Vec<String> = vec![];
    let mut updates : Vec<String> = vec![];
    
    let mut buffer = String::new();
    
    loop {
        let _bytes = reader.read_line(&mut buffer).unwrap();
                
        if buffer == "\r\n" {
            buffer.clear();
            break;
        }
        
        if buffer.find("\r\n").is_some() {
            buffer.truncate(buffer.len() - 2);
        }
        
        rules.push(buffer.clone());
        
        buffer.clear();
    }
    
    loop {
        let bytes = reader.read_line(&mut buffer).unwrap();
        
        if bytes == 0 {
            break;
        }
        
        if buffer.find("\r\n").is_some() {
            buffer.truncate(buffer.len() - 2);
        }
        
        updates.push(buffer.clone());
        
        buffer.clear();
    }
    
    (rules, updates)
}