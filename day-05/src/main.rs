use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn read_input(file_path: String) -> Result<(HashMap<u8, Vec<u8>>, Vec<Vec<u8>>), Error> {
    let reader = Box::new(BufReader::new(File::open(file_path)?));
    let mut update_rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut updates: Vec<Vec<u8>> = Vec::new();
    
    let regex_updates = Regex::new(r"((\d+),?)").unwrap();
    let regex_rules = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
    
    for lines in reader.lines() {
        let line = lines.unwrap();
        if regex_rules.is_match(&line) {     
            
            let line_values: [u8; 2] = line
                .split("|")
                .into_iter()
                .map(|v| v.parse().unwrap()).collect::<Vec<u8>>().try_into().unwrap();
            
            if let Some(pages) = update_rules.get_mut(&line_values[0]) {
                pages.push(line_values[1]);
            } else {
                update_rules.insert(line_values[0], vec![line_values[1]]);
            }
            continue;
        }
        if regex_updates.is_match(&line) {
            let line_values: Vec<u8> = line
                .split(",")
                .into_iter()
                .map(|v| v.parse().unwrap())
                .collect();
            
            updates.push(line_values);
            continue;
        }
    }
    return Ok((update_rules, updates));
}

fn part_one(update_rules: &HashMap<u8, Vec<u8>>, updates: &Vec<Vec<u8>>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    for update in updates {
        
        let pages_to_check = update.len();
        
        for (i, page) in update.iter().enumerate() {
            
            let mut pages_before: HashSet<u8> = HashSet::new();
            
            if i > 0 {
                pages_before = update[..i].into_iter().cloned().collect();
            }
            
            if let Some(rules_values) = update_rules.get(page) {
                let updated_pages: HashSet<_> = rules_values.into_iter().cloned().collect();
                let failed_updates: HashSet<_> = pages_before.intersection(&updated_pages).into_iter().copied().collect();
                if ! failed_updates.is_empty() {
                    println!("update: {:?} failed because {} must be updated before {:?}...", update, page, failed_updates);
                    break;
                }
            }                        
            
            if i == pages_to_check - 1 {
                result += update[i/2] as i32;
            }
        }
        
    }
    Ok(result)
}

fn part_two(update_rules: &HashMap<u8, Vec<u8>>, updates: &Vec<Vec<u8>>) -> Result<i32, Error> {
    let mut result: i32 = 0;
    Ok(result)
}

fn main() {
    let (update_rules, updates) = read_input("./input/part_one_input.txt".to_string()).unwrap();
    let res_p1: i32 = part_one(&update_rules, &updates).unwrap();  
    println!("Result Part One: {}", res_p1);

    // let res_p2: i32 = part_two(&update_rules, &updates).unwrap();
    // println!("Result Part Two: {}", res_p2);
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two, read_input};

    #[test]
    fn test_part_one() {        
        let (update_rules, updates) = read_input("./input/part_one_test_input.txt".to_string()).unwrap();
        assert_eq!(part_one(&update_rules, &updates).unwrap(), 143);
    }

    #[test]
    fn test_part_two() {
        // let update_rules, updates = read_input("./input/part_one_test_input.txt".to_string()).unwrap();
        // assert_eq!(part_two(&update_rules, &updates).unwrap(), 9);
    }
}
