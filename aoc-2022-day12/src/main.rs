use std::collections::{
     HashSet,
     VecDeque
    };

fn main() {
    let output = find_shortest_path(include_str!("../input.txt"),'S');
    println!("Ans1: {:?}", output);

    let mut output = find_shortest_path(include_str!("../input.txt"),'a');
    output.sort();
    println!("Ans2: {:?}", output.first());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn find_shortest_path(input: &str,starting_char:char) -> Vec<u64> {
    
    let mut grid: Vec<i64> = vec![];
    let row_size: usize = input.lines().next().unwrap().len();
    let col_size: usize = input.lines().count();
    let mut starting_points =vec![];
    for line in input.lines() {
        for c in line.trim().chars() {
            match c {
                _ if c == starting_char || c == 'S' => {
                    starting_points.push(grid.len());
                    if c == 'S' {
                        grid.push(('a' as u8 - 1) as i64)
                    }else{
                        grid.push(c as i64) 
                    }
                },
                'a'..='z' => grid.push(c as i64),
                'E'  => grid.push(('z' as u8 + 1) as i64),
                _ => (),
            }
        }
    }

    let mut depths:Vec<u64> = vec![];

    for starting_point in starting_points {

        let starting_point = Point { x: starting_point%row_size, y: starting_point/row_size };
        let mut queue: VecDeque<Point> = VecDeque::new();
        queue.push_back(starting_point);
        let mut visited = HashSet::new();
        let mut current_depth:usize = 0;
        let mut finished = false;

        while !queue.is_empty() && !finished {

            let mut level_size:usize = queue.len();

            while level_size > 0 {

                level_size -= 1;

                let current_point = queue.pop_front().unwrap();
                let current_index = current_point.x + current_point.y*row_size;
                let current_char = grid[current_index];

                if current_char == ('z' as u8+1) as i64 {
                    depths.push(current_depth as u64);
                    finished = true;
                } else  {

                    visited.insert(current_index);
                    
                    if current_point.x > 0 && (grid[current_point.x-1 + current_point.y*row_size]-current_char) <=1 {
                        if !visited.contains(&(current_point.x-1 + current_point.y*row_size)) {
                            let new_point = Point { x: current_point.x - 1, y: current_point.y };
                            if !queue.contains(&new_point){
                                queue.push_back(new_point);
                            }
                        }
                    }

                    if current_point.x < row_size - 1 && (grid[current_point.x+1 + current_point.y*row_size]-current_char) <= 1 {
                        if !visited.contains(&(current_point.x+1 + current_point.y*row_size)) {
                            let new_point = Point { x: current_point.x + 1, y: current_point.y };
                            if !queue.contains(&new_point){
                                queue.push_back(new_point);
                            }
                        }
                    }

                    if current_point.y > 0 && (grid[current_point.x + (current_point.y-1)*row_size]-current_char) <= 1{
                        if !visited.contains(&(current_point.x + (current_point.y-1)*row_size)) {
                            let new_point = Point { x: current_point.x, y: current_point.y - 1 };
                            if !queue.contains(&new_point){
                                queue.push_back(new_point);
                            }
                        }
                    }

                    if current_point.y < col_size - 1 && (grid[current_point.x+(current_point.y+1)*row_size]-current_char) <= 1 {
                        if !visited.contains(&(current_point.x+(current_point.y+1)*row_size)) {
                            let new_point = Point { x: current_point.x, y: current_point.y + 1 };
                            if !queue.contains(&new_point){
                                queue.push_back(new_point);
                            }
                        }
                    }
                }   
            }

            current_depth+=1;

        }
    }

    depths
}


#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";

    #[test]
    fn it_works() {
        let output = find_shortest_path(INPUT,'S');
        assert_eq!(output, vec![31]);
    }

}