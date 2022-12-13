use std::collections::HashSet;

fn main() {

   
    let mut rope = Rope{
        tail_history: HashSet::new(),
        head: Position{x: 0, y: 0},
        tail: Position{x: 0, y: 0},
    };

    include_str!("../input.txt")
            .lines()
            .map(parse_steps)
            .map(|x| rope.move_rope(x.unwrap()))
            .for_each(drop);
    
    println!("{}",rope.tail_history.len());

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
    tail_history: HashSet<Position>,
    head: Position,
    tail: Position,
}

impl Rope {
    
    fn move_rope(&mut self, steps: Vec<Step>) {

        for step in steps {
            match step.direction {
                Direction::Up => self.head.y += 1,
                Direction::Down => self.head.y -= 1,
                Direction::Left => self.head.x -= 1,
                Direction::Right => self.head.x += 1,
            }
            if self.head_is_not_touching_tail() {
                let (delta_x, delta_y) = self.get_distance();

                if delta_x >= 1 {
                    self.tail.x += (self.head.x- self.tail.x).clamp(-1,1);
                }
                if delta_y >= 1 {
                    self.tail.y += (self.head.y - self.tail.y).clamp(-1,1);
                }
                self.tail_history.insert(self.tail.clone());
            }
        }

        

    }

    fn get_distance(&self) -> (i64,i64)  {
        (
            (self.head.x- self.tail.x).abs() , 
            (self.head.y- self.tail.y).abs()
        )
    }

    fn head_is_not_touching_tail(&self) -> bool {
        let (delta_x, delta_y) = self.get_distance();
        delta_x > 1 || delta_y > 1
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

    use std::hash::Hash;

    use super::*;

    #[test]
    fn motion_parsing() {

        let _rope = Rope{
            tail_history: HashSet::new(),
            head: Position{x: 0, y: 0},
            tail: Position{x: 0, y: 0},
        };
        
        assert_eq!(parse_steps("D 142"), Some(vec![Step{direction: Direction::Down}; 142]));

    }

    #[test]
    fn rope_movement() {

        let mut rope = Rope{
            tail_history: HashSet::new(),
            head: Position{x: 0, y: 0},
            tail: Position{x: 0, y: 0},
        };

        rope.move_rope(parse_steps("R 1").unwrap());
        assert_eq!(rope.head, Position{x: 1, y: 0});
        assert_eq!(rope.tail, Position{x: 0, y: 0});

        rope.move_rope(parse_steps("R 1").unwrap());
        assert_eq!(rope.head, Position{x: 2, y: 0});
        assert_eq!(rope.tail, Position{x: 1, y: 0});

        rope.move_rope(parse_steps("R 1").unwrap());
        assert_eq!(rope.head, Position{x: 3, y: 0});
        assert_eq!(rope.tail, Position{x: 2, y: 0}); 

        rope.move_rope(parse_steps("R 1").unwrap());
        assert_eq!(rope.head, Position{x: 4, y: 0});
        assert_eq!(rope.tail, Position{x: 3, y: 0});

        rope.move_rope(parse_steps("U 1").unwrap());
        assert_eq!(rope.head, Position{x: 4, y: 1});
        assert_eq!(rope.tail, Position{x: 3, y: 0});


    }
    
    #[test]
    fn test_move_counter(){

        let mut rope = Rope{
            tail_history: HashSet::new(),
            head: Position{x: 0, y: 0},
            tail: Position{x: 0, y: 0},
        };
        
        rope.move_rope(parse_steps("R 4").unwrap());
        rope.move_rope(parse_steps("U 4").unwrap());
        rope.move_rope(parse_steps("L 3").unwrap());
        rope.move_rope(parse_steps("D 1").unwrap());
        rope.move_rope(parse_steps("R 4").unwrap());
        rope.move_rope(parse_steps("D 1").unwrap());
        rope.move_rope(parse_steps("L 5").unwrap());
        rope.move_rope(parse_steps("R 2").unwrap());

        assert_eq!(rope.tail_history.len(),13);

    } 

    #[test]
    fn test_head_is_not_touching(){
        let mut rope = Rope{
            tail_history: HashSet::new(),
            head: Position{x: 0, y: 0},
            tail: Position{x: 0, y: 0},
        };

        assert_eq!(rope.head_is_not_touching_tail(), false);
        rope.tail = Position{x: 1, y: 0};
        assert_eq!(rope.head_is_not_touching_tail(), false);
        rope.tail = Position{x: 0, y: 1};
        assert_eq!(rope.head_is_not_touching_tail(), false);
        rope.tail = Position{x: 1, y: 1};
        assert_eq!(rope.head_is_not_touching_tail(), false);
        rope.tail = Position{x: 2, y: 0};
        assert_eq!(rope.head_is_not_touching_tail(), true);
        rope.tail = Position{x: 0, y: 2};
        assert_eq!(rope.head_is_not_touching_tail(), true);


    }

    #[test]
    fn test_get_distance(){

        let mut rope = Rope{
            tail_history: HashSet::new(),
            head: Position{x: 0, y: 0},
            tail: Position{x: 0, y: 0},
        };
    
        include_str!("../test_input.txt").lines()
             .map(parse_steps)
             .map(|x| rope.move_rope(x.unwrap()))
             .for_each(drop);
        
        assert_eq!(rope.tail_history.len(), 13);
    
    }
}


