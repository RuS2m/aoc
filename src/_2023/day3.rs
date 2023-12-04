use crate::utils::NotImplementedErrorType;
use std::collections::HashSet;
use std::collections::HashMap;

const DOT_SYMBOL: char = '.';

fn sum_of_numbers_after_line(
    line: &str,
    prev_line_indices_to_numbers: &HashMap<i64, i64>,
    prev_line_symbols_locations: &HashSet<i64>
) -> (i64, HashMap<i64, i64>, HashSet<i64>) {
    let line_length = line.len() as i64;
    let mut numbers_sum_after_line = 0i64;
    let mut cur_line_indices_to_numbers: HashMap<i64, i64> = HashMap::new();
    let mut cur_line_symbols_locations: HashSet<i64> = HashSet::new();

    let mut previous_symbol = DOT_SYMBOL;
    let mut cur_number = String::new();
    let mut cur_number_index = -1i64;
    for (index, symbol) in line.chars().enumerate().map(|(i, s)| (i as i64, s)) {
        if index == 0 {
            previous_symbol = DOT_SYMBOL;
        }
        if symbol.is_digit(10) {
            if cur_number_index == -1i64 {
                cur_number_index = index;
            }
            cur_number.push(symbol);
        }
        
        if !symbol.is_digit(10) || index == line_length - 1 {
            if !cur_number.is_empty() {
                let cur_number_int = cur_number.parse::<i64>().unwrap();
                let mut number_was_counted = false;
                if previous_symbol != DOT_SYMBOL || (symbol != DOT_SYMBOL && !symbol.is_digit(10)) {
                    number_was_counted = true;
                }

                if !number_was_counted {
                    for i in (cur_number_index-1)..(index+1) {
                        if prev_line_symbols_locations.contains(&i) {
                            number_was_counted = true;
                            break;
                        }
                    }
                }

                if number_was_counted {
                    numbers_sum_after_line += cur_number_int;
                }
                else {
                    for i in (cur_number_index-1)..(index+1) {
                        let cur_line_number_at_index = cur_line_indices_to_numbers.get(&i).unwrap_or(&0);
                        cur_line_indices_to_numbers.insert(i, cur_line_number_at_index + cur_number_int);
                    }
                }

                cur_number_index = -1i64;
                cur_number.clear();
            }

            previous_symbol = symbol;

            if symbol != DOT_SYMBOL && !symbol.is_digit(10) {
                match prev_line_indices_to_numbers.get(&index) {
                    Some(num) => {
                        numbers_sum_after_line += num; },
                    _ => (),
                }
                cur_line_symbols_locations.insert(index);
            }
        }
    }

    (numbers_sum_after_line, cur_line_indices_to_numbers, cur_line_symbols_locations)
}

pub fn solve(input: Vec<&str>) -> Result<(Result<String, NotImplementedErrorType>, Result<String, NotImplementedErrorType>), NotImplementedErrorType> {
    let part1_result = part1(&input);
    let part2_result = part2(&input);

    Ok((part1_result, part2_result))
}

pub fn part1(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let mut prev_line_indices_to_numbers: HashMap<i64, i64> = HashMap::new();
    let mut prev_line_symbols_locations: HashSet<i64> = HashSet::new();
    let numbers_adjacent_to_symbol_sum = input.iter()
        .map(|line| {
            let (cur_sum, cur_indices_to_numbers, cur_symbols_locations) =
                sum_of_numbers_after_line(line, &prev_line_indices_to_numbers, &prev_line_symbols_locations);
            prev_line_indices_to_numbers = cur_indices_to_numbers;
            prev_line_symbols_locations = cur_symbols_locations;
            cur_sum
        })
        .sum::<i64>();

    Ok(numbers_adjacent_to_symbol_sum.to_string())
}

#[allow(unused_variables)]
pub fn part2(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    Err(NotImplementedErrorType)
}
