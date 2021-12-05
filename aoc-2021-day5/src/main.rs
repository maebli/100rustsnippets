use std::cmp::Ordering;

fn main() {

    let lines:Vec<Line>=INPUT
        .lines()
        .map(|x|{Line::from(x)})
        .filter(|x| { x.is_horizontal() | x.is_vertical()})
        .collect();

    let mut d = SquareMap::of_size(MAX_X_AND_Y_COORDINATE);

    d.consume_lines(lines);


    println!("{}",d.danger_point_count());
}

#[derive(Debug)]
struct SquareMap {
    size:usize,
    fields:Vec<u32>
}

impl SquareMap {

    fn of_size(s:usize) -> SquareMap {
        SquareMap { size:s,fields: vec![0; s*s] }
    }

    pub fn increment_danger(& mut self, p:Point){
        let i =( p.x+p.y*(self.size as u32) ) as usize;
        self.fields[i]+=1;
    }

    pub fn consume_line(&mut self, l:Line){

        let g = l.gradient();

        let range = match 0.cmp(&g.x) {
            Ordering::Less => l.start.x..(l.end.x + 1),
            Ordering::Greater => (l.end.x)..(l.start.x + 1),
            _ => 0..0
        };


        let mut p = l.start;

        for x in range {
            p.x = x;
            SquareMap::increment_danger(self, p);
        }

        let range = match 0.cmp(&g.y) {
            Ordering::Less => l.start.y..(l.end.y + 1),
            Ordering::Greater => (l.end.y)..(l.start.y + 1),
            _ => 0..0
        };

        for y in range {
            p.y = y;
            SquareMap::increment_danger(self, p);
        }


    }

    fn consume_lines(&mut self, l:Vec<Line>){
        for line in l {
            SquareMap::consume_line(self, line);
        }
    }

    fn danger_point_count(&self) -> u32 {
        self.fields.iter()
            .filter(|x| { **x > 1 })
            .count() as u32
    }
}

#[derive(Debug)]
struct Gradient {
    x:i32,
    y:i32
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x:u32,
    y:u32
}


impl Point {
    fn from(s:&str) -> Point {
        let x = s.split_once(",").unwrap();
        Point{ x: x.0.parse().unwrap(), y: x.1.parse().unwrap() }
    }
}

#[derive(Debug)]
struct Line{
    start:Point,
    end:Point
}

impl Line {
    fn from(s:&str) -> Line {
        let a = s.split_once(" -> ").unwrap();
        Line{ start: Point::from(a.0), end: Point::from(a.1) }
    }

    fn gradient(&self) -> Gradient{
        Gradient{ x: self.end.x as i32 - self.start.x as i32,
            y: self.end.y as i32 - self.start.y as i32
        }
    }

    fn is_horizontal(&self) ->bool {
        self.gradient().x == 0
    }

    fn is_vertical(&self) ->bool {
        self.gradient().y == 0
    }
}

const MAX_X_AND_Y_COORDINATE:usize = 10;
const INPUT:&str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";