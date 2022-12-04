fn main() {
    let x = include_str!("../input.txt").lines()
                .map(|line| {
                    let x = line.split_once(",");
                    let x = x.unwrap_or_else(|| panic!("Invalid input: {}", line));
                    (Assignment::from(x.0), Assignment::from(x.1))
                });

    let answer1 = x.clone()
                .filter(|(a, b)| a.is_subset_of(&b) || b.is_subset_of(&a))
                .count();

    let answer2 = x
                .filter(|(a, b)| a.overlaps(&b)).count();

    println!("Answer 1: {} and Answer 2: {}", answer1, answer2);
                

}


struct Assignment {
    start: u64,
    end: u64,
}

impl Assignment {

    fn is_subset_of(&self, other: &Assignment) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.start <= other.end && self.end >= other.start
    }
    
    fn from(line: &str) -> Assignment {
        let (start, end) = line
                .split_once("-")
                .unwrap_or_else(|| panic!("Invalid input: {}", line));
        Assignment {
            start: start.parse().unwrap_or(0),
            end: end.parse().unwrap_or(0),
        }
    }
}

// add tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subset_of() {
        let a = Assignment { start: 1, end: 10 };
        let b = Assignment { start: 1, end: 10 };
        assert!(a.is_subset_of(&b));

        let a = Assignment { start: 1, end: 10 };
        let b = Assignment { start: 1, end: 11 };
        assert!(a.is_subset_of(&b));

        let a = Assignment { start: 2, end: 10 };
        let b = Assignment { start: 1, end: 10 };
        assert!(a.is_subset_of(&b));
        assert!(!b.is_subset_of(&a));

    }
}