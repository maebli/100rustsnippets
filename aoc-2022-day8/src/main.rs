fn main() {

    let res1 = Map2D::from(include_str!("../input.txt")).count_visible_trees();
    println!("Result 1: {}", res1);
    
    let res2 = Map2D::from(include_str!("../input.txt")).max_scenic_score();
    println!("Result 2: {}", res2);
}


struct Map2D {
    data: Vec<Vec<u64>>,
}

impl Map2D {

   fn from(input: &str) -> Self {
       let x = input.lines()
                                   .map(|line| line.trim()
                                                         .chars()
                                                         .map(|c| c.to_digit(10)
                                                         .unwrap() as u64)
                                                         .collect())
                                   .collect();
       Self { data: x }
   }

   fn is_visible(&self, x: usize, y: usize) -> bool {
       
       if x == 0 || y == 0 || x == self.data.len() - 1 || y == self.data[x].len() - 1 {
           return true;
       }
       let l=*self.data[x][..y].iter().max().unwrap() < self.data[x][y];
       let r=*self.data[x][y+1..].iter().max().unwrap() < self.data[x][y];
       let t=self.data[..x].iter().map(|v| v[y]).max().unwrap() < self.data[x][y];
       let b=self.data[x+1..].iter().map(|v| v[y]).max().unwrap() < self.data[x][y];
       l||r||t||b
   }

   fn scenic_score(&self, x: usize, y: usize) -> u64 {
        
        if x == 0 || y == 0 || x == self.data.len() - 1 || y == self.data[x].len() - 1 {
            return 0;
        }

        let len= self.data[x].len();
        let l:u64 = 1+self.data[x][1..y].iter().rev().map_while(|&v| if v < self.data[x][y] { Some(v) } else { None }).count() as u64;
        let r:u64 = 1+self.data[x][y+1..len-1].iter().map_while(|&v| if v < self.data[x][y] { Some(v) } else { None }).count() as u64;
        let t:u64 = 1+self.data[1..x].iter().rev().map(|v| v[y]).map_while(|v| if v < self.data[x][y] { Some(v) } else { None }).count() as u64;
        let b:u64 = 1+self.data[x+1..len-1].iter().map(|v| v[y]).map_while(|v| if v < self.data[x][y] { Some(v) } else { None }).count() as u64;
        l*r*t*b
   }
 
   fn count_visible_trees(&self) -> usize {
       self.data.iter()
                .enumerate()
                .map(|(x, row)| row.iter().enumerate()
                .filter(|(y, _)| self.is_visible(x, *y))
                .count())
                .sum()
   }

   fn max_scenic_score(&self) -> u64 {
       self.data.iter()
                .enumerate()
                .map(|(x, row)| row.iter().enumerate()
                .map(|(y, _)| self.scenic_score(x, y))
                .max()
                .unwrap())
                .max()
                .unwrap()
    }

 
}

#[cfg(test)]
mod tests {

    use crate::*;

    const TEST_INPUT: &str = 
   "30373
    25512
    65332
    33549
    35390";


    #[test]
    fn test_genrate_map() {
        let map = Map2D::from(TEST_INPUT);
        assert_eq!(map.data.len(), 5);
        assert_eq!(map.data[0].len(), 5);
        println!("{:?}", map.data);
    }

    #[test]
    fn test_height_map() {
        assert_eq!(Map2D::from(TEST_INPUT).count_visible_trees(), 21);
    }

    #[test]
    fn test_is_visible(){
        assert_eq!(Map2D::from(TEST_INPUT).is_visible(0, 0), true);
        assert_eq!(Map2D::from(TEST_INPUT).is_visible(0, 1), true);
        assert_eq!(Map2D::from(TEST_INPUT).is_visible(1, 1), true);
        assert_eq!(Map2D::from(TEST_INPUT).is_visible(1, 2), true);
        assert_eq!(Map2D::from(TEST_INPUT).is_visible(2, 2), false);
    }

    #[test]
    fn test_scenic_scores(){
        assert_eq!(Map2D::from(TEST_INPUT).scenic_score(1,2),4);
        assert_eq!(Map2D::from(TEST_INPUT).scenic_score(3,2),8);
    }

}