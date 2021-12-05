use std::cmp;
use std::cmp::Ordering;

fn main() {

    let lines:Vec<Line>=INPUT
        .lines()
        .map(|x|{Line::from(x)})
        // Uncomment for challenge 1 result
        //.filter(|x|{x.is_horizontal()|x.is_vertical()})
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
        let i =( p.x + p.y * (self.size as u32) ) as usize;
        self.fields[i]+=1;
    }

    pub fn consume_line(&mut self, l:Line){

        let g = l.gradient();

        let mut p = l.start;

        for _ in 0..(l.length()+1) {
            SquareMap::increment_danger(self,p);
            p.x = (p.x as i32 + g.x) as u32;
            p.y = (p.y as i32 + g.y) as u32;
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

    fn gradient(&self) -> Gradient {

        let dx = self.end.x as i32 - self.start.x as i32;
        let dy  = self.end.y as i32 - self.start.y as i32;

        let dx = match 0.cmp(&dx) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            _ => 0
        };

        let dy = match 0.cmp(&dy) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            _ => 0
        };

        Gradient{
            x: dx,
            y: dy
        }
    }

    fn length(&self) -> u32 {
        cmp::max((self.end.x as i32 - self.start.x as i32).abs(),
            (self.end.y as i32 - self.start.y as i32).abs()) as u32

    }

    fn is_horizontal(&self) ->bool {
        self.gradient().x == 0
    }

    fn is_vertical(&self) ->bool {
        self.gradient().y == 0
    }
}

const MAX_X_AND_Y_COORDINATE:usize = 10; // change to 1000 for challenge input
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