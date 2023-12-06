use crate::utils::NotImplementedErrorType;
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn solve(input: &Vec<&str>) -> Result<(Result<String, NotImplementedErrorType>, Result<String, NotImplementedErrorType>), NotImplementedErrorType> {
    let part1_result = part1(input);
    let part2_result = part2(input);

    Ok((part1_result, part2_result))
}

pub fn part1(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let overall_score = input.iter()
        .map(|c| card_score(c))
        .sum::<i64>()
        .to_string();
    Ok(overall_score)
}

pub fn part2(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let gained_scratchpads: Vec<usize> = input.iter()
        .map(|c| number_of_matches_per_card(c))
        .map(|n| usize::try_from(n).unwrap())
        .collect::<Vec<usize>>();

    let initial_scratchpad_count = gained_scratchpads.len();
    let mut scratchpad_counts_list = vec![1i64; initial_scratchpad_count];
    let mut total_scratchpads = 0i64;

    for (i, matches) in gained_scratchpads.iter().enumerate() {
        total_scratchpads += scratchpad_counts_list[i];
        for duplicate_scratchpad_index in (i+1)..(i+matches+1) {
            if duplicate_scratchpad_index < initial_scratchpad_count {
                scratchpad_counts_list[duplicate_scratchpad_index]+=scratchpad_counts_list[i];
            }
        }
    }
    Ok(total_scratchpads.to_string())
}

fn number_of_matches_per_card(card: &str) -> i64 {
    let re = Regex::new("^Card\\s+(?<card_id>\\d+): (?<winning_numbers>[\\d|\\s]+) \\| (?<elfs_numbers>[\\d+|\\s]+)$").unwrap();
    let Some(caps) = re.captures(card) else { panic!("could not match the regex with the card {}", card) };
    
    let winning_numbers: HashSet<&str> = HashSet::from_iter(caps["winning_numbers"].split_whitespace().filter(|s| !s.is_empty()));
    caps["elfs_numbers"].split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|num| winning_numbers.contains(num) as i64)
        .sum::<i64>()
}

fn card_score(card: &str) -> i64 {
    let matches = number_of_matches_per_card(card); 
    if matches == 0 {
        0
    } else {
        2i64.pow((matches - 1) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_day4_part1_example() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
    
        let result = part1(&input).expect("Failed to solve part 1 example of day 4");
        
        assert_eq!(result, "13".to_string());
    }
    
    #[test]
    fn test_2023_day4_part2_example() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
    
        let result = part2(&input).expect("Failed to solve part 2 example of day 4");
        
        assert_eq!(result, "30".to_string());
    }
    
    #[test]
    fn test_2023_day4_input() {
         let file_content = read_to_string("resources/2023-04.txt")
             .expect("Failed to read file");
    
         let input: Vec<&str> = file_content.lines().collect();
    
        let (answer1, answer2) = solve(&input).expect("Failed to solve input of day 4");
        
        assert_eq!(answer1.unwrap(), "26346".to_string());
        assert_eq!(answer2.unwrap(), "8467762".to_string());
    }
}
