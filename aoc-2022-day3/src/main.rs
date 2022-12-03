use std::collections::{HashSet};

fn main() {

    let answer:u64 = include_str!("../input.txt").lines()
            .map(find_only_uniqe_item_in_rucksack)
            .map(get_priority_of_item)
            .sum();
    
    println!("{}", answer);
}

fn find_only_uniqe_item_in_rucksack(rucksack:&str) -> u8 {

    let left_compartment:HashSet<char> = rucksack[..(rucksack.len()-1)/2+1].chars().collect();
    let right_compartment:HashSet<char> = rucksack[(rucksack.len()-1)/2+1..].chars().collect();
    
    *left_compartment.intersection(&right_compartment)
                    .next()
                    .unwrap_or_else(|| panic!("No unique item found in rucksack: {}", rucksack))
                    as u8
}

fn get_priority_of_item(item:u8) -> u64{
    match item {
        x if item >= b'A' && item <= b'Z' => (x - b'A' + 27).into(),
        x if item >= b'a' && item <= b'z' => (x - b'a' + 1).into(),
        _ => panic!("Invalid item")
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_that_if_single_item_found_in_both_compoartents_correctly_found() {
        let test_input:Vec<(&str,u8)> = vec![
            ("vJrwpWtwJgWrhcsFMMfFFhFp",b'p'),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",b'L'),
            ("PmmdzqPrVvPwwTWBwg",b'P'),
            ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",b'v'),
            ("ttgJtRGJQctTZtZT",b't'),
            ("CrZsJsPPZsGzwwsLwLmpwMDw",b's')
        ];
        let _ = test_input.iter()
                    .inspect(|x|assert_eq!(find_only_uniqe_item_in_rucksack(x.0),x.1))
                    .collect::<Vec<_>>();
    }

    #[test]
    fn test_priority_calc_of_item() {
        let test_input:Vec<(u8,u64)> = vec![
            (b'A',27),
            (b'a',1),
            (b'Z',52),
            (b'z',26)
        ];
        let _ = test_input.iter()
                        .inspect(|x|assert!(get_priority_of_item(x.0)==x.1))
                        .collect::<Vec<_>>();
    }

}