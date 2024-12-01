use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_input(file_path: String) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let reader = Box::new(BufReader::new(File::open(file_path)?));
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    for lines in reader.lines() {
        let line = lines.unwrap();
        let line_values: Vec<i32> = line
            .split("   ")
            .into_iter()
            .map(|v| v.parse().unwrap())
            .collect();
        list_one.push(line_values[0].clone());
        list_two.push(line_values[1].clone());
    }

    return Ok((list_one, list_two));
}

fn day_01_part_one(list_one: &mut Vec<i32>, list_two: &mut Vec<i32>) -> Result<i32, Error> {
    list_one.sort();
    list_two.sort();

    let mut difference = Vec::with_capacity(list_one.len());

    for i in 0..list_one.len() {
        difference.push((list_one[i] - list_two[i]).abs());
    }

    let result: i32 = difference.iter().sum();
    Ok(result)
}

fn day_01_part_two(list_one: &Vec<i32>, list_two: &Vec<i32>) -> Result<i32, Error> {
    let mut similarity = Vec::with_capacity(list_one.len());
    for number_one in list_one {
        let counts: i32 = list_two
            .iter()
            .filter(|&&x| x == *number_one)
            .count()
            .try_into()
            .unwrap();
        similarity.push(counts * number_one);
    }

    let result: i32 = similarity.iter().sum();
    Ok(result)
}

fn main() {
    let (mut list_one, mut list_two) = read_input("part_one_input.txt".to_string()).unwrap();
    let result = day_01_part_one(&mut list_one, &mut list_two);
    println!("Result Part One: {}", result.unwrap());

    let (list_one, list_two) = read_input("part_one_input.txt".to_string()).unwrap();
    let result = day_01_part_two(&list_one, &list_two);
    println!("Result Part Two: {}", result.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::{day_01_part_one, day_01_part_two};

    #[test]
    fn test_day_01_part_one() {
        let mut array_one = vec![3, 4, 2, 1, 3, 3];
        let mut array_two = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(day_01_part_one(&mut array_one, &mut array_two).unwrap(), 11);
    }

    #[test]
    fn test_day_01_part_two() {
        let array_one = vec![3, 4, 2, 1, 3, 3];
        let array_two = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(day_01_part_two(&array_one, &array_two).unwrap(), 31);
    }
}
