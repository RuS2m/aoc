use crate::utils::NotImplementedErrorType;
use std::cmp::Ordering;
use std::collections::HashMap;

lazy_static! {
    static ref CARD_VALUES: HashMap<char, i64> = {
        let mut m = HashMap::new();
        m.insert('A', 14);
        m.insert('K', 13);
        m.insert('Q', 12);
        m.insert('J', 11);
        m.insert('T', 10);
        m.insert('9', 9);
        m.insert('8', 8);
        m.insert('7', 7);
        m.insert('6', 6);
        m.insert('5', 5);
        m.insert('4', 4);
        m.insert('3', 3);
        m.insert('2', 2);
        m
    };

    static ref CARD_WITH_JOKER_VALUES: HashMap<char, i64> = {
        let mut m = CARD_VALUES.clone();
        m.insert('J', 1);
        m
    };
}

pub fn solve(input: &Vec<&str>) -> Result<(Result<String, NotImplementedErrorType>, Result<String, NotImplementedErrorType>), NotImplementedErrorType> {
    let part1_result = part1(input);
    let part2_result = part2(input);

    Ok((part1_result, part2_result))
}

pub fn part1(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let mut cards_hands = input.iter().map(|line| {
        let hand_with_bid = line.split_whitespace().collect::<Vec<&str>>();
        let hand = hand_with_bid[0].to_string();
        let bid = hand_with_bid[1].parse::<i64>().unwrap();
        CamelCardsHand { cards: hand, bid: bid}
    }).collect::<Vec<CamelCardsHand>>();

    cards_hands.sort_by(|hand, other_hand| hand.cmp(&other_hand));

    let answer = cards_hands.iter()
        .enumerate()
        .map(|(i, card_hand)| ((i+1) as i64) * card_hand.bid)
        .sum::<i64>();
    Ok(answer.to_string())
}

pub fn part2(input: &[&str]) -> Result<String, NotImplementedErrorType> {
    let mut cards_hands = input.iter().map(|line| {
        let hand_with_bid = line.split_whitespace().collect::<Vec<&str>>();
        let hand = hand_with_bid[0].to_string();
        let bid = hand_with_bid[1].parse::<i64>().unwrap();
        CamelCardsHand { cards: hand, bid: bid}
    }).collect::<Vec<CamelCardsHand>>();

    cards_hands.sort_by(|hand, other_hand| hand.cmp_with_jokers(&other_hand));

    let answer = cards_hands.iter()
        .enumerate()
        .map(|(i, card_hand)| ((i+1) as i64) * card_hand.bid)
        .sum::<i64>();
    Ok(answer.to_string())
}

fn card_value(card: &char) -> i64 {
    *CARD_VALUES.get(card).unwrap_or(&0i64)
}

fn card_value_with_jokers(card: &char) -> i64 {
    *CARD_WITH_JOKER_VALUES.get(card).unwrap_or(&0i64)
}

fn combination_power_from_card_frequencies(card_frequency: &HashMap<char, i64>) -> i64 {
    let mut frequencies: Vec<&i64> = card_frequency.values().clone().collect();
    frequencies.sort();
    frequencies.reverse();

    if *frequencies[0] == 5 {
        7i64
    } else if *frequencies[0] == 4 {
        6i64
    } else if *frequencies[0] == 3 && *frequencies[1] == 2 {
        5i64
    } else if *frequencies[0] == 3 && *frequencies[1] == 1 {
        4i64
    } else if *frequencies[0] == 2 && *frequencies[1] == 2 {
        3i64
    } else if *frequencies[0] == 2 && *frequencies[1] == 1 {
        2i64
    } else {
        1i64
    }
}

fn combination_power(hand: &str) -> i64 {
    let mut card_frequency: HashMap<char, i64> = HashMap::new();
    for card in hand.chars() {
        *card_frequency.entry(card).or_insert(0i64) += 1i64;
    }

    combination_power_from_card_frequencies(&card_frequency)
}

fn potential_combination_power(hand: &str) -> i64 {
    let mut card_frequency: HashMap<char, i64> = HashMap::new();
    for card in hand.chars() {
        *card_frequency.entry(card).or_insert(0i64) += 1i64;
    }

    let jokers_amount = card_frequency.remove(&'J').unwrap_or(0i64);
    let max_frequency_card = card_frequency.iter().max_by_key(|entry | entry.1).map(|(key, _)| *key).unwrap_or('J');

    if let Some(val) = card_frequency.get_mut(&max_frequency_card) {
        *val += jokers_amount;
    } else {
        // if removed card was Joker and there were no other cards -- it should be replaced with
        // maximum amount of cards (5)
        card_frequency.insert('J', 5);
    }

    combination_power_from_card_frequencies(&card_frequency)
}

#[derive(Debug)]
struct CamelCardsHand {
    cards: String,
    bid: i64,
}

impl CamelCardsHand {
    fn abstract_cmp<'a>(&self, other: &Self, combination_power_fn: &'a dyn Fn(&str) -> i64, card_value_fn: &'a dyn Fn(&char) -> i64) -> Ordering {
        let cards_combination_power = combination_power_fn(&self.cards);
        let cards_combination_power_other = combination_power_fn(&other.cards);
        
        if cards_combination_power > cards_combination_power_other {
            Ordering::Greater
        } else if cards_combination_power < cards_combination_power_other { 
            Ordering::Less
        } else {
            for (i, card) in self.cards.chars().enumerate() {
                let other_card = other.cards.chars().nth(i).unwrap();
                if card_value_fn(&card) > card_value_fn(&other_card) {
                    return Ordering::Greater;
                } else if card_value_fn(&card) < card_value_fn(&other_card) { 
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.abstract_cmp(other, &combination_power, &card_value)
    }

    fn cmp_with_jokers(&self, other: &Self) -> Ordering {
        self.abstract_cmp(other, &potential_combination_power, &card_value_with_jokers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2023_day7_part1_example() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];
    
        let result = part1(&input).expect("Failed to solve part 1 example of day 7");
        
        assert_eq!(result, "6440".to_string());
    }
    
    #[test]
    fn test_2023_day7_part2_example() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];
    
        let result = part2(&input).expect("Failed to solve part 2 example of day 7");
        
        assert_eq!(result, "5905".to_string());
    }
    
    #[test]
    fn test_2023_day7_input() {
         let file_content = std::fs::read_to_string("resources/2023-07.txt")
             .expect("Failed to read file");
    
         let input: Vec<&str> = file_content.lines().collect();
    
        let (answer1, answer2) = solve(&input).expect("Failed to solve input of day 7");
        
        assert_eq!(answer1.unwrap(), "251058093".to_string());
        assert_eq!(answer2.unwrap(), "249781879".to_string());
    }
}
