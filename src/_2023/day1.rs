use crate::utils::NotImplementedErrorType;
use std::fs;

fn first_and_last_digit_number(line: &str) -> i64 {
    let digits = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>();
    let mut first_digit = 0i64;
    let mut last_digit = 0i64;

    if let Some(first_digit_char) = digits.first() {
        first_digit = first_digit_char.to_digit(10).unwrap() as i64;
    }

    if let Some(last_digit_char) = digits.last() {
        last_digit = last_digit_char.to_digit(10).unwrap() as i64;
    }

    return first_digit*10 + last_digit;
}

fn first_and_last_spelled_digit_number(line: &str) -> i64 {
    let mut digits = Vec::new();
    for (i, c) in line.chars().enumerate() {
        let cur_slice = &line[i..];
        if c.is_digit(10) {
            digits.push(c.to_digit(10).unwrap() as i64);
        }
        if cur_slice.starts_with("one") {
            digits.push(1);
        }
        if cur_slice.starts_with("two") {
            digits.push(2);
        }
        if cur_slice.starts_with("three") {
            digits.push(3);
        }
        if cur_slice.starts_with("four") {
            digits.push(4);
        }
        if cur_slice.starts_with("five") {
            digits.push(5);
        }
        if cur_slice.starts_with("six") {
            digits.push(6);
        }
        if cur_slice.starts_with("seven") {
            digits.push(7);
        }
        if cur_slice.starts_with("eight") {
            digits.push(8);
        }
        if cur_slice.starts_with("nine") { 
            digits.push(9);
        }
    }

    let mut first_digit = 0i64;
    let mut last_digit = 0i64;

    if let Some(first_digit_char) = digits.first() {
        first_digit = *first_digit_char;
    }

    if let Some(last_digit_char) = digits.last() {
        last_digit = *last_digit_char;
    }

    return first_digit*10 + last_digit;
}

pub fn solve(input: &Vec<&str>) -> Result<(Result<String, NotImplementedErrorType>, Result<String, NotImplementedErrorType>), NotImplementedErrorType> {
    let part1_result = part1(input);
    let part2_result = part2(input);

    Ok((part1_result, part2_result))
}

pub fn part1(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let sum = input.iter()
        .map(|x| first_and_last_digit_number(x))
        .sum::<i64>();

    Ok(sum.to_string())
}

pub fn part2(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let sum = input.iter()
        .map(|x| first_and_last_spelled_digit_number(x))
        .sum::<i64>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_day1_part1_example() {
        let input = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];
    
        let result = part1(&input).expect("Failed to solve part 1 example of day 1");
        
        assert_eq!(result, "142".to_string());
    }
    
    #[test]
    fn test_2023_day1_part2_example() {
        let input = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
    
        let result = part2(&input).expect("Failed to solve part 2 example of day 1");
        
        assert_eq!(result, "281".to_string());
    }
    
    #[test]
    fn test_2023_day1_input() {
         let file_content = fs::read_to_string("resources/2023-01.txt")
             .expect("Failed to read file");
    
         let input: Vec<&str> = file_content.lines().collect();
    
        let (answer1, answer2) = solve(&input).expect("Failed to solve input of day 1");
        
        assert_eq!(answer1.unwrap(), "53334".to_string());
        assert_eq!(answer2.unwrap(), "52834".to_string());
    }
}
