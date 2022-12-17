use std::{collections::HashSet, borrow::{Borrow, BorrowMut}};

fn main() {
   
    let mut rope1 = Rope{
        history: HashSet::new(),
        positions: vec![Position{x: 0, y: 0};10],
    };

    include_str!("../input.txt")
            .lines()
            .map(parse_steps)
            .map(|x| rope1.move_rope(x.unwrap()))
            .for_each(drop);
    
    println!("Length of rope 1 {}",rope1.history.len());

    let mut rope2 = Rope{
        history: HashSet::new(),
        positions: vec![Position{x: 0, y: 0};2],
    };

    include_str!("../input.txt")
            .lines()
            .map(parse_steps)
            .map(|x| rope2.move_rope(x.unwrap()))
            .for_each(drop);
    
    println!("Length of rope 2{}",rope2.history.len());

}


#[derive(Debug,PartialEq,Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Motion {
    steps: Vec<Step>
}

#[derive(Debug, PartialEq,Clone)]
struct Step {
    direction: Direction,
}

#[derive(Debug, PartialEq,Clone,Hash,Eq)]
struct Position {
    x: i64,
    y: i64,
}
#[derive(Debug, PartialEq,Eq,Clone)]
struct Rope {
    history: HashSet<Position>,
    positions: Vec<Position>,
}

impl Position {

    fn get_distance(a:&Position,b:&Position) -> (i64,i64)  {
        (
            (a.x- b.x).abs() , 
            (a.y- b.y).abs()
        )
    }

    fn head_is_not_touching_tail(a:&Position,b:&Position) -> bool {
        let (delta_x, delta_y) = Self::get_distance(a,b);
        delta_x > 1 || delta_y > 1
    }
}

impl Rope {
    
    fn move_rope(&mut self, steps: Vec<Step>) {

        for step in steps {

            let mut head = self.positions.first().unwrap().clone();

            match step.direction {
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            }
            self.positions[0]=head;

            self.positions.iter()
                          .enumerate()
                          .map(|i| (i.0,i.0+1))
                          .collect::<Vec<(_,_)>>()
                          .iter()
                          .for_each(|(a,b)| {
                                let head = self.positions.get(*a).unwrap();
                                if let Some(tail) = self.positions.get(*b){
                                    
                                    let mut tail = tail.clone();
                                    if Position::head_is_not_touching_tail(&head,&tail) {
                        
                                        let (delta_x, delta_y) = Position::get_distance(&head,&tail);
                    
                                        if delta_x >= 1 {
                                            tail.x += (head.x - tail.x).clamp(-1,1);
                                        }
                                        if delta_y >= 1 {
                                            tail.y += (head.y - tail.y).clamp(-1,1);
                                        }
                                    }
                                    self.positions[*b] = tail;
                                }

                          });
            self.history.insert(self.positions.last().unwrap().clone());
        }
        
    }

        

}


fn parse_steps(input:&str) -> Option<Vec<Step>> {

    let distance = input[2..].parse::<u64>().ok()?;
    let direction = match input.chars().nth(0)?{
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction"),
    };

    Some(vec![Step{direction}; distance as usize])
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn motion_parsing() {

        let _rope = Rope{
            history: HashSet::new(),
            positions: vec![Position{x: 0, y: 0};2],
        };
        
        assert_eq!(parse_steps("D 142"), Some(vec![Step{direction: Direction::Down}; 142]));

    }

    #[test]
    fn rope_movement() {

        let mut rope = Rope{
            history: HashSet::new(),
            positions: vec![Position{x: 0, y: 0};2],
        };

        rope.move_rope(parse_steps("R 1").unwrap());
        assert_eq!(*rope.positions.first().unwrap(), Position{x: 1, y: 0});
        assert_eq!(*rope.positions.last().unwrap(), Position{x: 0, y: 0});

        rope.move_rope(parse_steps("R 1").unwrap());
        assert_eq!(*rope.positions.first().unwrap(), Position{x:2, y: 0});
        assert_eq!(*rope.positions.last().unwrap(), Position{x: 1, y: 0});

        rope.move_rope(parse_steps("R 1").unwrap());
        assert_eq!(*rope.positions.first().unwrap(), Position{x: 3, y: 0});
        assert_eq!(*rope.positions.last().unwrap(), Position{x: 2, y: 0});

        rope.move_rope(parse_steps("R 1").unwrap());
        assert_eq!(*rope.positions.first().unwrap(), Position{x: 4, y: 0});
        assert_eq!(*rope.positions.last().unwrap(), Position{x: 3, y: 0});

        rope.move_rope(parse_steps("U 1").unwrap());
        assert_eq!(*rope.positions.first().unwrap(), Position{x:4, y: 1});
        assert_eq!(*rope.positions.last().unwrap(), Position{x: 3, y: 0});

        rope.move_rope(parse_steps("U 1").unwrap());
        assert_eq!(*rope.positions.first().unwrap(), Position{x:4, y: 2});
        assert_eq!(*rope.positions.last().unwrap(), Position{x: 4, y: 1});

    }
    
    #[test]
    fn test_move_counter(){

        let mut rope = Rope{
            history: HashSet::new(),
            positions: vec![Position{x: 0, y: 0};2],
        };
        
        rope.move_rope(parse_steps("R 4").unwrap());
        rope.move_rope(parse_steps("U 4").unwrap());
        rope.move_rope(parse_steps("L 3").unwrap());
        rope.move_rope(parse_steps("D 1").unwrap());
        rope.move_rope(parse_steps("R 4").unwrap());
        rope.move_rope(parse_steps("D 1").unwrap());
        rope.move_rope(parse_steps("L 5").unwrap());
        rope.move_rope(parse_steps("R 2").unwrap());

        assert_eq!(rope.history.len(),13);

    } 

    #[test]
    fn test_head_is_not_touching(){

        assert_eq!(Position::head_is_not_touching_tail(&Position{x:0,y:0}, &Position{x:0,y:0}),false);
        assert_eq!(Position::head_is_not_touching_tail(&Position{x:1,y:0}, &Position{x:0,y:0}),false);
        assert_eq!(Position::head_is_not_touching_tail(&Position{x:0,y:1}, &Position{x:0,y:0}),false);
        assert_eq!(Position::head_is_not_touching_tail(&Position{x:1,y:1}, &Position{x:0,y:0}),false);
        assert_eq!(Position::head_is_not_touching_tail(&Position{x:2,y:0}, &Position{x:0,y:0}),true);

    }

    #[test]
    fn test_get_distance(){

        let mut rope = Rope{
            history: HashSet::new(),
            positions: vec![Position{x: 0, y: 0};2],
        };
    
        include_str!("../test_input.txt").lines()
             .map(parse_steps)
             .map(|x| rope.move_rope(x.unwrap()))
             .for_each(drop);
        
        assert_eq!(rope.history.len(), 13);
    
    }
}


