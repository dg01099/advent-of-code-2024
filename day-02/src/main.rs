use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_input(file_path: String) -> Result<Vec<Vec<i32>>, Error> {
    let reader = Box::new(BufReader::new(File::open(file_path)?));
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for lines in reader.lines() {
        let line = lines.unwrap();
        let line_values: Vec<i32> = line
            .split(" ")
            .into_iter()
            .map(|v| v.parse().unwrap())
            .collect();
        reports.push(line_values);
    }

    return Ok(reports);
}

fn part_one(reports: &Vec<Vec<i32>>) -> Result<i32, Error> {
    let mut save_reports: i32 = 0;
    for report in reports {
        let mut level_diff: Vec<i32> = Vec::new();
        for (index, value) in report.iter().enumerate() {
            if index > 0 {
                level_diff.push(value - report[index - 1]);
            }
        }
        if (level_diff.iter().filter(|&&x| x < 0).count() == level_diff.len())
            | (level_diff.iter().filter(|&&x| x > 0).count() == level_diff.len())
        {
            if level_diff.iter().filter(|&&x| x.abs() > 3).count() == 0 {
                save_reports += 1;
            }
        }
    }
    Ok(save_reports)
}

fn get_faulty_report(report: &Vec<i32>) -> HashSet<usize> {
    
    let mut faulty_level: HashSet<usize> = HashSet::new();
    let mut diffs: Vec<i32> = Vec::new();

    let mut sign_change: HashSet<usize> = HashSet::new();

    for (index, value) in report.iter().enumerate() {
        if index > 0 {
            let _level_diff = value - report[index - 1];
            diffs.push(_level_diff);
            if (_level_diff.abs() < 1) | (_level_diff.abs() > 3) {
                if index == report.len() - 1 {                    
                    faulty_level.insert(index);
                } else {                    
                    faulty_level.insert(index - 1);
                }
            }
        }
    }
    // println!("Diffs: {:?}", diffs);
    for (index, diff) in diffs.iter().enumerate() {
        if (index > 0) && (index < (diffs.len() - 1)) {
            if (*diff < 0) && (diffs[index - 1] > 0) {
                if index == 1 {
                    sign_change.insert(0);
                } else {
                    sign_change.insert(index);
                }
            }
        } else if index == (diffs.len() - 1) {            
            if ((*diff < 0) && (diffs[index - 1] > 0)) || ((*diff > 0) && (diffs[index - 1] < 0)) {
                sign_change.insert(diffs.len());
            }
        }
    }
    // println!("Sig Change: {:?}", sign_change);
    // println!("Faulty: {:?}", faulty_level);
    let result: HashSet<_> = faulty_level
        .union(&sign_change)
        .into_iter()
        .copied()
        .collect();
    return result;
}

fn part_two(reports: &Vec<Vec<i32>>) -> Result<i32, Error> {
    let mut save_reports: i32 = 0;

    for report in reports {
        let faulty_report_index = get_faulty_report(report);

        if faulty_report_index.len() == 0 {
            save_reports += 1;
        } else if faulty_report_index.len() == 1 {
            let mut error_dumped_report = report.clone();
            println!("Vorher: {:?}", error_dumped_report);
            for remove_index in faulty_report_index {
                error_dumped_report.remove(remove_index);
            }
            println!("Nachher: {:?}", error_dumped_report);
            if (get_faulty_report(&error_dumped_report)).len() == 0 {
                save_reports += 1;
                println!("Save")
            }
            println!("----");
        }
    }

    Ok(save_reports)
}

fn main() {
    let reports = read_input("./input/part_one.txt".to_string()).unwrap();
    let result = part_one(&reports);
    println!("Result Part One: {}", result.unwrap());

    let reports = read_input("./input/part_one.txt".to_string()).unwrap();
    let result = part_two(&reports);
    println!("Result Part Two: {}", result.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::{get_faulty_report, part_one, part_two, read_input};
    use std::collections::HashSet;

    #[test]
    fn test_part_one() {
        let reports = read_input("./input/part_one_test.txt".to_string()).unwrap();
        assert_eq!(part_one(&reports).unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        let reports = read_input("./input/part_one_test.txt".to_string()).unwrap();
        assert_eq!(part_two(&reports).unwrap(), 4);
    }

    #[test]
    fn test_get_faulty_report() {
        let report_03 = vec![58, 57, 58, 56, 54];
        assert_eq!(get_faulty_report(&report_03), HashSet::from([2]));

        let report_01 = vec![83, 86, 85, 84, 81, 79, 78];
        assert_eq!(get_faulty_report(&report_01), HashSet::from([0]));

        let report_02 = vec![78, 79, 82, 85, 88, 85];
        assert_eq!(get_faulty_report(&report_02), HashSet::from([5]));

        let report_02 = vec![71, 67, 64, 63, 62, 64];
        assert_eq!(get_faulty_report(&report_02), HashSet::from([0, 5]));

        let report_02 = vec![89, 88, 85, 84, 82, 81, 79, 75];
        assert_eq!(get_faulty_report(&report_02), HashSet::from([7]));
    }
}
