use crate::utils::NotImplementedErrorType;
use std::collections::BTreeMap;

#[allow(unused_variables)]
pub fn solve(input: &Vec<&str>) -> Result<(Result<String, NotImplementedErrorType>, Result<String, NotImplementedErrorType>), NotImplementedErrorType> {
    let part1_result = part1(input);
    let part2_result = part2(input);

    Ok((part1_result, Err(NotImplementedErrorType)))
}

pub fn part1(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let mut seeds: Vec<i64> = Vec::new();
    
    if let Some(first_line) = input.first() {
        // 6 is the length of "seeds:" 
        seeds = first_line[6..]
            .split_whitespace()
            .filter_map(|seed| seed.parse::<i64>().ok())
            .collect();
        
    }

    run_through_transformation_pipeline(input, &mut seeds, transform_seeds);

    seeds.sort();
    Ok((*seeds.first().unwrap()).to_string())
}

pub fn part2(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let mut seed_chunks: Vec<(i64, i64)> = Vec::new();
    
    if let Some(first_line) = input.first() {
        // 6 is the length of "seeds:" 
        seed_chunks = first_line[6..]
            .split_whitespace()
            .filter_map(|seed| seed.parse::<i64>().ok())
            .collect::<Vec<i64>>()
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();
    }

    run_through_transformation_pipeline(input, &mut seed_chunks, transform_seed_chunks);

    seed_chunks.sort_by_key(|chunk| chunk.1);
    Ok(seed_chunks.first().unwrap().1.to_string())
}

fn run_through_transformation_pipeline<T>(input: &[&str], seeds: &mut Vec<T>, transform_seeds_fun: impl Fn(&mut Vec<T>, &BTreeMap<i64, (i64, i64)>) -> ()) {
    let mut seeds_mappings: BTreeMap<i64, (i64, i64)> = BTreeMap::new();

    for line in input.iter().skip(1) {
        // transform seeds
        if line.is_empty() && !seeds_mappings.is_empty() {
            transform_seeds_fun(seeds, &seeds_mappings);
            seeds_mappings.clear();
        }
        // create new mappings
        else if !line.is_empty() && !line.ends_with(':') {
            let parts = line.split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<i64>>();
            seeds_mappings.insert(parts[1], (parts[0], parts[2]));
        }
    }

    transform_seeds_fun(seeds, &seeds_mappings);
}

#[allow(unused_variables)]
fn transform_seed_chunks(seed_chunks: &mut Vec<(i64, i64)>, seeds_mappings: &BTreeMap<i64, (i64, i64)>) {
    // Do nothing
}
// draft of second part solution (I tried to come up with super optimized algorithm and died from
// number of edge cases :*( )
/*fn transform_seed_chunks(seed_chunks: &mut Vec<(i64, i64)>, seeds_mappings: &BTreeMap<i64, (i64, i64)>) {
    let mut transformed_seed_chunks: Vec<(i64, i64)> = Vec::new();
    println!("Transformation rules {:?}", seeds_mappings);

    for seed_chunk in seed_chunks.iter() {
        let (seed_range_start, seed_range_length) = *seed_chunk; 
        println!("Transforming seed chunk ({}..{})\n", seed_range_start, seed_range_start+seed_range_length);
        let mut current_start = seed_range_start;
        let range_end = seed_range_start + seed_range_length;

        while current_start <= range_end {
            println!("\tIterating through range: current start is {current_start}");
            let mut current_start_upper_bound = seeds_mappings.range(..(current_start+1)).rev();
            if let Some((&source_range_start, &(transformation_range_start, transformation_range_length))) = current_start_upper_bound.next()) {
                println!("\t\tClosest transformation rule is {source_range_start} -> ({transformation_range_start}, {transformation_range_length})");
                // if any element in the range will be transformed
                if current_start <= source_range_start + transformation_range_length {
                    let elements_to_transform = transformation_range_length.min(range_end - current_start + 1); 
                    let transformed_start = current_start + transformation_range_start - source_range_start;
                    transformed_seed_chunks.push((transformed_start, elements_to_transform));
                    println!("\t\tTransformed {} elements from {} to {}", elements_to_transform, transformed_start, transformed_start + elements_to_transform);
                    current_start += elements_to_transform;
                }
                // elements are too small
                else {
                    // TODO: make it a loop
                    // move iterator
                    // if next iterator exists
                    if let Some(&next_source_start, &next_transformation_info) = current_start_upper_bound.next_back() {
                        let last_nontranformed_element = range_end.min(next_source_start);
                        let elements_nontransformed = last_nontranformed_element - current_start + 1;
                        transformed_seed_chunks.push((current_start, elements_nontransformed));
                        println!("\t\tTransformation rule is out of range, new nontransformed chunk of {} elements from {} to {}", elements_nontransformed, current_start, last_nontranformed_element);
                    current_start = last_nontranformed_element + 1;
                    }
                    // next iterator doesn't exist
                    else {
                        transform_seed_chunks.push((current_start, range_end - current_start + 1));
                        current_start = range_end + 1;
                        println!("\t\tALL transformation rules out of range, non transforming the rest of the chunk from {} to {}", current_start, range_end);
                    }
                }
                println!("\t\tNew start is {current_start}");
            } else {
                println!("\t\tNo mappings that satisfy condition found, new untransformed chunk of {} elements from {} to {}", range_end - current_start + 1, current_start, range_end);
                transformed_seed_chunks.push((current_start, range_end - current_start + 1));
                current_start = range_end + 1;
            }
        }
    }

    *seed_chunks = transformed_seed_chunks;
}*/

fn transform_seeds(seeds: &mut Vec<i64>, seeds_mappings: &BTreeMap<i64, (i64, i64)>) {
    for seed in seeds.iter_mut() {
        let optional_transformation_info = seeds_mappings.range(..(*seed+1)).last().map(|(&key, &value)| (key, value));
        if let Some((source_range_start, transformation_range_info)) = optional_transformation_info {
            let (transformation_range_start, transformation_range_length) = transformation_range_info;

            if *seed <= source_range_start + transformation_range_length {
                *seed += transformation_range_start - source_range_start;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_day5_part1_example() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4"
        ];
    
        let result = part1(&input).expect("Failed to solve part 1 example of day 5");
        
        assert_eq!(result, "35".to_string());
    }
    
    #[test]
    fn test_2023_day5_input() {
         let file_content = std::fs::read_to_string("resources/2023-05.txt")
             .expect("Failed to read file");
    
         let input: Vec<&str> = file_content.lines().collect();
    
        let (answer1, _) = solve(&input).expect("Failed to solve input of day 5");
        
        assert_eq!(answer1.unwrap(), "199602917".to_string());
    }
}
