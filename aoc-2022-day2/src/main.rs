use std::cmp::Ordering;


const POINTS_FOR_WIN:u32 = 6;
const POINTS_FOR_DRAW:u32 = 3;
const POINTS_FOR_LOSS:u32 = 0;


fn main() {
   println!("{} ", GameMove::get_sum_part1(include_str!("../input.txt")));
   println!("{} ", GameMove::get_sum_part2(include_str!("../input.txt")));
}
#[derive(PartialOrd, PartialEq, Debug,Eq,Clone)]
enum GameMove {
    Paper,
    Rock,
    Scissors,
}



impl GameMove {
    fn from(i:char) -> Self {
        match i {
            'X'|'A' => GameMove::Rock,
            'Y'|'B' => GameMove::Paper,
            'Z'|'C' => GameMove::Scissors,
            _ => panic!("Invalid input"),
        }
    }

    fn value(&self) -> u32 {
        match self {
            GameMove::Rock => 1,
            GameMove::Paper => 2,
            GameMove::Scissors => 3,
        }
    }

    fn read_moves(input:&str) -> Vec<((std::cmp::Ordering,u32),(std::cmp::Ordering,u32))>{
        input.lines().map(|line| {
            let mut parts = line.split_whitespace();
            let oponent_move = GameMove::from(parts.next().unwrap().chars().next().unwrap());
            let second_input = parts.next().unwrap().chars().next().unwrap();
            let your_move_part1 = GameMove::from(second_input);
            let your_move_part2 = oponent_move.from_ordering(second_input);
            let res_part2 = your_move_part2.cmp(&oponent_move);
            let res_part1= your_move_part1.cmp(&oponent_move);
            let points_part2=match res_part2 {
                Ordering::Equal => (res_part2,your_move_part2.value()+POINTS_FOR_DRAW),
                Ordering::Greater => (res_part2,your_move_part2.value()+POINTS_FOR_WIN),
                Ordering::Less => (res_part2,your_move_part2.value()+POINTS_FOR_LOSS),
            };
            let points_part1=match res_part1 {
                Ordering::Equal => (res_part1,your_move_part1.value()+POINTS_FOR_DRAW),
                Ordering::Greater => (res_part1,your_move_part1.value()+POINTS_FOR_WIN),
                Ordering::Less => (res_part1,your_move_part1.value()+POINTS_FOR_LOSS),
            };
            (points_part1,points_part2)
        }).collect()
    }

    fn get_sum_part1(input:&str) -> u32 {
        GameMove::read_moves(input).iter().map(|(a,_)| a.1).sum()
    }

    fn get_sum_part2(input:&str) -> u32 {
        GameMove::read_moves(input).iter().map(|(_,a)| a.1).sum()
    }

    fn from_ordering(&self,i:char) -> GameMove {
        match i {
            'Z' => {
                match *self {
                    GameMove::Rock => GameMove::Paper,
                    GameMove::Paper => GameMove::Scissors,
                    GameMove::Scissors => GameMove::Rock,
                }
            }
            'Y' => self.clone(),
            'X' => match *self {
                GameMove::Rock => GameMove::Scissors,
                GameMove::Paper => GameMove::Rock,
                GameMove::Scissors => GameMove::Paper,
            },
            _ => panic!("Invalid input"),
        }
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
        assert_eq!(GameMove::Rock.cmp(&GameMove::Paper), Ordering::Less);
        assert_eq!(GameMove::Rock.cmp(&GameMove::Rock), Ordering::Equal);
        assert_eq!(GameMove::Scissors.cmp(&GameMove::Paper), Ordering::Greater);
        assert_eq!(GameMove::Scissors.cmp(&GameMove::Rock), Ordering::Less);
        assert_eq!(GameMove::Scissors.cmp(&GameMove::Scissors), Ordering::Equal);
    }
    #[test]
    fn testInput(){
        assert_eq!(GameMove::get_sum_part1(TEST_INPUT), 15);
    }
}
