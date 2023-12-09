use crate::utils::NotImplementedErrorType;

pub fn solve(input: &Vec<&str>) -> Result<(Result<String, NotImplementedErrorType>, Result<String, NotImplementedErrorType>), NotImplementedErrorType> {
    let part1_result = part1(input);
    let part2_result = part2(input);

    Ok((part1_result, part2_result))
}

pub fn part1(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    // Times word consists of 5 symbols
    let times = split_into_integers(&input[0][5..]);
    // Distance word consists of 10 symbols
    let record_distances = split_into_integers(&input[1][8..]);
    let mut answer = 1i64;
    for (i, record_distance) in record_distances.iter().enumerate() {
        // for sure there should be some record, otherwise the answer would be 0
        let first_record_breakage = binary_search_first_record_break(times[i], *record_distance);
        let times_record_broken = times[i] - 2*first_record_breakage + 1;
        println!("Record for time {} and distance {} was broken by {} ways", times[i], *record_distance, times_record_broken);
        answer *= times_record_broken;
    }
    Ok(answer.to_string())
}

pub fn part2(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let time = combine_into_integer_ignoring_whitespaces(&input[0][5..]);
    let record_distance = combine_into_integer_ignoring_whitespaces(&input[1][10..]);
    let first_record_breakage = binary_search_first_record_break(time, record_distance);
    let times_record_broken = time - 2*first_record_breakage + 1;
    Ok(times_record_broken.to_string())
}

fn split_into_integers(input: &str) -> Vec<i64> {
    input.split_whitespace()
        .filter_map(|distance| distance.parse::<i64>().ok())
        .collect::<Vec<i64>>()
}

fn combine_into_integer_ignoring_whitespaces(input: &str) -> i64 {
    input.split_whitespace()
        .filter(|distance| !distance.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap()
}

fn binary_search_first_record_break(time: i64, record_distance: i64) -> i64 {
    let lower_half: i64 = if time % 2 == 0 { time / 2 } else { (time - 1)/2 };

    let mut low: i64 = 1;
    let mut high: i64 = lower_half;

    while low < high {
        let mid = if (high - low) % 2 == 0 { ((high - low) / 2) + low } else { ((high - low - 1)/ 2) + low };

        let val = mid * (time - mid);

        if val <= record_distance {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_day6_part1_example() {
        let input = vec![
            "Time:      7  15   30",
            "Distance:  9  40  200"
        ];
    
        let result = part1(&input).expect("Failed to solve part 1 example of day 6");
        
        assert_eq!(result, "288".to_string());
    }
    
    #[test]
    fn test_2023_day6_part2_example() {
        let input = vec![
            "Time:      7  15   30",
            "Distance:  9  40  200"
        ];
    
        let result = part2(&input).expect("Failed to solve part 1 example of day 6");
        
        assert_eq!(result, "71503".to_string());
    }
    
    #[test]
    fn test_2023_day6_input() {
         let file_content = std::fs::read_to_string("resources/2023-06.txt")
             .expect("Failed to read file");
    
         let input: Vec<&str> = file_content.lines().collect();
    
        let (answer1, answer2) = solve(&input).expect("Failed to solve input of day 6");
        
        assert_eq!(answer1.unwrap(), "608902".to_string());
        assert_eq!(answer2.unwrap(), "46173809".to_string());
    }
}
