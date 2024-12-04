use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_input(file_path: String) -> Result<Vec<Vec<char>>, Error> {
    let reader = Box::new(BufReader::new(File::open(file_path)?));
    let mut content: Vec<Vec<char>> = Vec::new();
    
    for lines in reader.lines() {
        let line = lines.unwrap().chars().collect();
        content.push(line)
    }
    return Ok(content);
}

fn part_one(content: &Vec<Vec<char>>) -> Result<u16, Error> {
    let mut result: u16 = 0;
    let possible_positions: [[[i16; 2]; 4]; 8] = [
        [
            [0, 0], [1, 0], [2, 0], [3, 0]          // left-to-right
        ], [
            [-3, 0], [-2, 0], [-1, 0], [0, 0]       // right-to-left
        ], [
            [0, 0], [0, -1], [0, -2], [0, -3]       // up-to-down
        ], [
            [0, 0], [0, 1], [0, 2], [0, 3]          // down-to-up
        ], [
            [0, 0], [1, 1], [2, 2], [3, 3]          // up-left-to-down-right
        ], [
            [0, 0], [-1, -1], [-2, -2], [-3, -3]    // down-right-to-up-left
        ], [
            [0, 0], [1, -1], [2, -2], [3, -3]       // down-left-to-up-right
        ], [
            [0, 0], [-1, 1], [-2, 2], [-3, 3]       // up-right-to-down-left
        ]
    ];
    let match_phrase: [char; 4] = ['X', 'M', 'A', 'S'];
    let match_phrase_rev: [char; 4] = [ 'S', 'A', 'M', 'X'];
    
    let max_y: i16 = content.len().try_into().unwrap();
    let mut max_x: i16;
    for (y, row) in content.iter().enumerate() {
        max_x = row.len().try_into().unwrap();
            
        for (x, sign) in row.iter().enumerate() {
            
            if *sign != 'X' {
                continue;
            }
            
            let mut positions_to_check: Vec<[[u8; 2]; 4]> = Vec::new();
            
            'direction_loop: for direction in possible_positions {
                let mut new_pos: [[u8; 2]; 4] = [[0; 2]; 4];
                let mut finished = false;
                
                for (i, pos) in direction.iter().enumerate() {
                    let new_x: i16 = (x as i16) + pos[0];
                    let new_y: i16 = (y as i16) + pos[1];
                    
                    if new_x < 0 || new_x >= max_x || new_y < 0 || new_y >= max_y {
                        break;
                    }                    
                    
                    new_pos[i] = [new_x.try_into().unwrap(), new_y.try_into().unwrap()];
                    if i == direction.len() - 1 {
                        finished = true;
                    }
                }
                if finished {
                    positions_to_check.push(new_pos.clone());
                }
                
            }
            
            println!("\nX:{} | Y:{}", x, y);
            // println!("{:?}", positions_to_check);
            
            'position_check: for positions in positions_to_check {
                let mut phrase: [char; 4] = ['.'; 4];
                for (i, position) in positions.iter().enumerate() {
                    let pos_x = position[0] as usize;
                    let pos_y = position[1] as usize;
                    let current_char = content[pos_y][pos_x];
                    
                    phrase[i] = current_char;
                    if (current_char != match_phrase[i]) && (current_char != match_phrase_rev[i])  {
                        break;
                    }
                }
                if (phrase == match_phrase) || (phrase == match_phrase_rev) {   
                    println!("{:?} -> {:?}", phrase, positions);                 
                    result += 1;
                } else {                    
                    // println!("{:?} -> {:?}", positions, phrase);
                }
            }           
        }
    }
    
    Ok(result)
}

fn part_two(content: &Vec<Vec<char>>) -> Result<u16, Error> {
    let mut result: u16 = 0;
        let possible_positions: [[[i16; 2]; 4]; 8] = [
            [
                [-1, -1], [0, 0], [2, 0], [3, 0]          // left-to-right
            ], [
                [-3, 0], [-2, 0], [-1, 0], [0, 0]       // right-to-left
            ], [
                [0, 0], [0, -1], [0, -2], [0, -3]       // up-to-down
            ], [
                [0, 0], [0, 1], [0, 2], [0, 3]          // down-to-up
            ], [
                [0, 0], [1, 1], [2, 2], [3, 3]          // up-left-to-down-right
            ], [
                [0, 0], [-1, -1], [-2, -2], [-3, -3]    // down-right-to-up-left
            ], [
                [0, 0], [1, -1], [2, -2], [3, -3]       // down-left-to-up-right
            ], [
                [0, 0], [-1, 1], [-2, 2], [-3, 3]       // up-right-to-down-left
            ]
        ];
        let match_phrase: [char; 4] = ['M', 'A', 'S'];
        let match_phrase_rev: [char; 4] = [ 'S', 'A', 'M'];
        
        let max_y: i16 = content.len().try_into().unwrap();
        let mut max_x: i16;
        for (y, row) in content.iter().enumerate() {
            max_x = row.len().try_into().unwrap();
                
            for (x, sign) in row.iter().enumerate() {
                
                if *sign != 'A' {
                    continue;
                }
                
                let mut positions_to_check: Vec<[[u8; 2]; 4]> = Vec::new();
                
                'direction_loop: for direction in possible_positions {
                    let mut new_pos: [[u8; 2]; 4] = [[0; 2]; 4];
                    let mut finished = false;
                    
                    for (i, pos) in direction.iter().enumerate() {
                        let new_x: i16 = (x as i16) + pos[0];
                        let new_y: i16 = (y as i16) + pos[1];
                        
                        if new_x < 0 || new_x >= max_x || new_y < 0 || new_y >= max_y {
                            break;
                        }                    
                        
                        new_pos[i] = [new_x.try_into().unwrap(), new_y.try_into().unwrap()];
                        if i == direction.len() - 1 {
                            finished = true;
                        }
                    }
                    if finished {
                        positions_to_check.push(new_pos.clone());
                    }
                    
                }
                
                println!("\nX:{} | Y:{}", x, y);
                // println!("{:?}", positions_to_check);
                
                'position_check: for positions in positions_to_check {
                    let mut phrase: [char; 4] = ['.'; 4];
                    for (i, position) in positions.iter().enumerate() {
                        let pos_x = position[0] as usize;
                        let pos_y = position[1] as usize;
                        let current_char = content[pos_y][pos_x];
                        
                        phrase[i] = current_char;
                        if (current_char != match_phrase[i]) && (current_char != match_phrase_rev[i])  {
                            break;
                        }
                    }
                    if (phrase == match_phrase) || (phrase == match_phrase_rev) {   
                        println!("{:?} -> {:?}", phrase, positions);                 
                        result += 1;
                    } else {                    
                        // println!("{:?} -> {:?}", positions, phrase);
                    }
                }           
            }
        }
    Ok(result)
}

fn main() {
    let content = read_input("./input/part_one_input.txt".to_string()).unwrap();
    let res_p1: u16 = part_one(&content).unwrap();  
    println!("Result Part One: {}", res_p1);

    // let res_p2: i32 = part_two(&content.as_str()).unwrap();
    // println!("Result Part Two: {}", res_p2);
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two, read_input};

    #[test]
    fn test_part_one() {        
        let content = read_input("./input/part_one_test_input.txt".to_string()).unwrap();
        assert_eq!(part_one(&content).unwrap(), 18);
    }

    #[test]
    fn test_part_two() {
        // let content = read_input("./input/part_one_test_input.txt".to_string()).unwrap();
        // println!("{}", content)
        // assert_eq!(part_two(&input).unwrap(), 48);
    }
}
