use std::cmp::Ordering;

fn main() {
   let input = include_str!("../input.txt");
}
#[derive(PartialOrd, PartialEq, Debug,Eq)]
enum GameMove {
    Paper,
    Rock,
    Scissors,
}

impl GameMove {
    fn from(i:char) -> Self {
        match i {
            'X'|'A' => GameMove::Paper,
            'Y'|'B' => GameMove::Rock,
            'Z'|'C' => GameMove::Scissors,
            _ => panic!("Invalid input"),
        }
    }

    fn value(&self) -> i32 {
        match self {
            GameMove::Paper => 3,
            GameMove::Rock => 6,
            GameMove::Scissors => 2,
        }
    }

    fn read_moves(input:&str) -> Vec<(std::cmp::Ordering,i32)>{
        input.lines().map(|line| {
            let mut parts = line.split_whitespace();
            let a = GameMove::from(parts.next().unwrap().chars().next().unwrap());
            let b = GameMove::from(parts.next().unwrap().chars().next().unwrap());
            let res = a.cmp(&b);
            match res {
                Ordering::Equal => (res,0),
                Ordering::Greater => (res,b.value()),
                Ordering::Less => (res,0),
            }
        }).collect()
    }
}

impl Ord for GameMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other {
            GameMove::Paper => match self{
                GameMove::Paper => std::cmp::Ordering::Equal,
                GameMove::Rock => std::cmp::Ordering::Less,
                GameMove::Scissors => std::cmp::Ordering::Greater,
            },
            GameMove::Rock => match self {
                GameMove::Paper => std::cmp::Ordering::Greater,
                GameMove::Rock => std::cmp::Ordering::Equal,
                GameMove::Scissors => std::cmp::Ordering::Less,
            },
            GameMove::Scissors => match self {
                GameMove::Paper => std::cmp::Ordering::Less,
                GameMove::Rock => std::cmp::Ordering::Greater,
                GameMove::Scissors => std::cmp::Ordering::Equal,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    const TEST_INPUT: &str = "A Y
    B X
    C Z";

    #[test]
    fn testType() {
        assert_eq!(GameMove::Paper.cmp(&GameMove::Paper), Ordering::Equal);
        assert_eq!(GameMove::Paper.cmp(&GameMove::Rock), Ordering::Greater);
        assert_eq!(GameMove::Paper.cmp(&GameMove::Scissors), Ordering::Less);
        assert_eq!(GameMove::Rock.cmp(&GameMove::Scissors), Ordering::Greater);
    }
    #[test]
    fn testInput(){
        let k=GameMove::read_moves(TEST_INPUT);
        print!("{:?}",k)
        
    }
}