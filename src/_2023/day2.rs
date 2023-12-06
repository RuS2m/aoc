use std::fs;
use std::collections::HashMap;
use crate::utils::NotImplementedErrorType;

lazy_static! {
    static ref CUBE_LIMITS: HashMap<&'static str, i64> = {
        let mut m = HashMap::new();
        m.insert("red", 12);
        m.insert("green", 13);
        m.insert("blue", 14);
        m
    };
}

fn game_id_if_valid(line: &str) -> i64 {
    let (game_id, game_steps) = parse_game(line);
    
    if game_steps.split(';').map(|cube_set| cube_set.split(", ").map(|cube_description| cube_combination_is_valid(cube_description)).all(|val| val)).all(|val| val) {
        game_id
    } else {
        0i64
    }
}

fn parse_game(line: &str) -> (i64, &str) {
    let game_parts = line.split(":").collect::<Vec<&str>>();
    let game_id = game_parts[0].split(' ').last().map(|x| x.parse::<i64>().unwrap()).unwrap();
    let game_steps = game_parts[1];
    (game_id, game_steps)
}

fn cube_combination_is_valid(cube_description: &str) -> bool {
    let cube_description_parts = cube_description.trim().split(' ').collect::<Vec<&str>>(); 
    let cube_count = cube_description_parts.first().map(|x| x.parse::<i64>().unwrap()).unwrap();
    let cube_color = cube_description_parts.last().unwrap();
    CUBE_LIMITS.get(cube_color).map_or(false, |&cube_limit| cube_limit >= cube_count)
}

fn power_of_minimum_set(line: &str) -> i64 {
    let (_, game_steps) = parse_game(line);
    
    let mut min_red = 0i64;
    let mut min_green = 0i64;
    let mut min_blue = 0i64;
    for game in game_steps.split(';').collect::<Vec<&str>>() {
        let (red, green, blue) = game.split(", ").map(|cube_combination| cube_combination_values(cube_combination)).fold((0, 0, 0), |(red_val1, green_val1, blue_val1), (red_val2, green_val2, blue_val2)| { (red_val1 + red_val2, green_val1 + green_val2, blue_val1 + blue_val2) });
        if red >= min_red {
            min_red = red;
        }
        if green >= min_green {
            min_green = green;
        }
        if blue >= min_blue {
            min_blue = blue;
        }
    }
    return min_red * min_green * min_blue;
}

fn cube_combination_values(cube_description: &str) -> (i64, i64, i64) {
    let cube_description_parts = cube_description.trim().split(' ').collect::<Vec<&str>>();
    let cube_count = cube_description_parts.first().map(|x| x.parse::<i64>().unwrap()).unwrap();
    let cube_color = cube_description_parts.last().unwrap();
    match *cube_color {
        "red" => (cube_count, 0, 0),
        "green" => (0, cube_count, 0),
        "blue" => (0, 0, cube_count),
        _ => (0, 0, 0),
    }
}

pub fn solve(input: &Vec<&str>) -> Result<(Result<String, NotImplementedErrorType>, Result<String, NotImplementedErrorType>), NotImplementedErrorType> {
    let part1_result = part1(input);
    let part2_result = part2(input);

    Ok((part1_result, part2_result))
}

pub fn part1(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let sum = input.iter()
        .map(|x| game_id_if_valid(x))
        .sum::<i64>();

    Ok(sum.to_string())
}

pub fn part2(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let sum = input.iter()
        .map(|x| power_of_minimum_set(x))
        .sum::<i64>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_day2_part1_example() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
    
        let result = part1(&input).expect("Failed to solve part 1 example of day 1");
        
        assert_eq!(result, "8".to_string());
    }
    
    #[test]
    fn test_2023_day2_part2_example() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
    
        let result = part2(&input).expect("Failed to solve part 2 example of day 1");
        
        assert_eq!(result, "2286".to_string());
    }
    
    #[test]
    fn test_2023_day2_input() {
         let file_content = fs::read_to_string("resources/2023-02.txt")
             .expect("Failed to read file");
    
         let input: Vec<&str> = file_content.lines().collect();
    
        let (answer1, answer2) = solve(&input).expect("Failed to solve input of day 1");
        
        assert_eq!(answer1.unwrap(), "2285".to_string());
        assert_eq!(answer2.unwrap(), "77021".to_string());
    }
}
