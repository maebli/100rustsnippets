use std::collections::HashMap;

use itertools::Itertools;

fn main() {

    let mut x = HashMap::new();
    let x = include_str!("../input.txt").lines()
        .fold(&mut x, | acc, line| {
            match line.chars().nth(0) {
                Some('m') => move_crates(acc, line),
                _ => init_crates(acc, line),
            };
            acc
        });

    let mut res = String::new();
    for k in 1..x.len()+1 {
        let v = x.get(&(k as u64)).unwrap();
        res.push(*v.last().unwrap());
    }

    println!("{}", res);
    
}

fn init_crates<'a>(stacks: &'a mut HashMap<u64,Vec<char>>, input:&str) -> &'a mut HashMap<u64,Vec<char>> {

    // append char to input
    let input = format!("{} ", input);
    let x = input.chars()
         .tuples().map(|(_,b,_,_)| b)
         .enumerate()
         .filter(|(_,c)| *c != ' ' && c.is_alphabetic())
         .fold(stacks, |acc, (i, b)| {
                acc.entry((i+1) as u64).or_insert(vec![]).insert(0, b);
                acc
        });
    x
}

fn move_crates<'a>(stacks: &'a mut HashMap<u64,Vec<char>>, input:&str) -> &'a mut HashMap<u64,Vec<char>> {
   
    let x=input.split(" ")
            .filter(|x| x.parse::<u64>().is_ok())
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

    let (number, origin, destination) = (x[0], x[1], x[2]);

    let crates = stacks.get_mut(&origin).unwrap();
    let mut crates = crates.split_off(crates.len()-number as usize);
    // comment out this line for second answer with crate mover 9001
    crates.reverse();
    stacks.get_mut(&destination).unwrap().append(& mut crates);


    stacks
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_init_crates() {

        let tests = [
            ("            [F] [C] [H] [F] [W] [P]\n"
            ,HashMap::from([(4,vec!['F']),
                           (5,vec!['C']),
                           (6,vec!['H']),
                           (7,vec!['F']),
                           (8,vec!['W']),
                           (9,vec!['P'])
                           ])
            ),
            ("[F] [C] [H] [F] [W] [P]\n"
            ,HashMap::from([(1,vec!['F']),
                           (2,vec!['C']),
                           (3,vec!['H']),
                           (4,vec!['F']),
                           (5,vec!['W']),
                           (6,vec!['P'])
                           ])
            ),
            ("    [C] [H] [F] [W] [P]\n"
            ,HashMap::from([
                           (2,vec!['C']),
                           (3,vec!['H']),
                           (4,vec!['F']),
                           (5,vec!['W']),
                           (6,vec!['P'])
                           ])
            ),
            (" 1   2   3 \n"
            ,HashMap::from([])
            ),
                           ];
        for test in tests {
            assert_eq!(super::init_crates(&mut HashMap::new(),test.0), &test.1);
        }
    
    }

    #[test]
    fn test_move_crates() {

        let test = vec![
            (
                (HashMap::from([(1,vec!['Z','N','D']),(2,vec!['M','C']), (3,vec!['P'])],),"move 1 from 1 to 2"),
                (HashMap::from([(1,vec!['Z','N']),(2,vec!['M','C','D']),(3,vec!['P'])]))
            ),
            (
                (HashMap::from([(1,vec![]),(2,vec!['M','C']), (3,vec![])],),"move 2 from 2 to 1"),
                (HashMap::from([(1,vec!['C','M']),(2,vec![]),(3,vec![])]))
            )
        ];

        for mut test in test {
            assert_eq!(super::move_crates(& mut test.0.0, test.0.1), &test.1);
        }

    }
}