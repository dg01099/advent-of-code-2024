use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use regex::Regex;

pub fn read_input(file_path: String) -> Result<String, Error> {
    let reader = Box::new(BufReader::new(File::open(file_path)?));
    let mut content: String = "".to_string();
    for lines in reader.lines() {
        let line = lines.unwrap();
        content += &line.to_string();
    }
    return Ok(content);
}

fn part_one(command: &str) -> Result<i32, Error> {
    let mut result: i32 = 0;
    let re = Regex::new(r"mul\((?P<m1>\d{1,3}),(?P<m2>\d{1,3})\)").unwrap();
    let captures = re.captures_iter(command);
    for capture in captures {
        let m1: i32 = capture.name("m1").unwrap().as_str().parse().unwrap();
        let m2: i32 = capture.name("m2").unwrap().as_str().parse().unwrap();        
        println!("Found multiplication function: {}x{}", m1, m2);
        result += m1*m2
    }
    Ok(result)
}

fn part_two(command: &str) -> Result<i32, Error> {
    let mut result: i32 = 0;
    let mut enabled: bool = true;
    
    let re = Regex::new(r"((?P<switch>do(n't)?\(\)).*?)?mul\((?P<m1>\d{1,3}),(?P<m2>\d{1,3})\)").unwrap();
    let captures = re.captures_iter(command);
    
    for capture in captures {
        if let Some(switch) = capture.name("switch") {            
            if switch.as_str() == "do()" {
                println!("Switch enabled");
                enabled = true;
            } else if switch.as_str() == "don't()" {            
                println!("Switch disabled");
                enabled = false;
            }
        }
        let m1: i32 = capture.name("m1").unwrap().as_str().parse().unwrap();
        let m2: i32 = capture.name("m2").unwrap().as_str().parse().unwrap();                
        println!("Found multiplication function: {}x{} and current switch state is {}", m1, m2, enabled);
        if enabled {            
            result += m1*m2
        }
    }
    Ok(result)
}

fn main() {
    let content = read_input("./input/part_one_input.txt".to_string()).unwrap();
    let res_p1: i32 = part_one(&content.as_str()).unwrap();  
    println!("Result Part One: {}", res_p1);

    let res_p2: i32 = part_two(&content.as_str()).unwrap();
    println!("Result Part Two: {}", res_p2);
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_one(&input).unwrap(), 161);
    }

    #[test]
    fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_two(&input).unwrap(), 48);
    }
}
