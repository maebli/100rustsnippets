use std::fs;

fn main() {
    // read input.txt file
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt file");
    let res = get_elf_with_most_calories(input.as_str());
    println!("The elf with the most calories has a score of {}", res.0);
    println!("The sum of calories of the top three elves is {}", res.0+res.1+res.2);
}

fn get_elf_with_most_calories(input:&str) -> (u32,u32,u32) {
    
    let mut calories = input.lines().into_iter().fold(vec![], |mut acc, line| {
        match line.trim().parse::<u32>() {
            Ok( n) => {
                let n=*acc.pop().get_or_insert(0)+n;
                acc.push(n);
            }
            _ => {
                acc.push(0)
            }
        }
        acc
    });

    calories.sort();
    
    (calories.pop().unwrap_or(0),
     calories.pop().unwrap_or(0),
     calories.pop().unwrap_or(0))

}


#[cfg(test)]
mod tests {

    use crate::{get_elf_with_most_calories};

    const INPUT: &str = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";

    #[test]
    fn test_function_get_elf_with_most_calories() {
        assert_eq!(get_elf_with_most_calories(INPUT).0, 24000);
    }

    #[test]
    fn test_function_get_some_of_top_three(){
        let i=get_elf_with_most_calories(INPUT);
        assert_eq!(i.0+i.1+i.2, 45000);
    }

}