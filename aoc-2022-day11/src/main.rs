use std::fmt;

const NUMBER_OF_ROUNDS: u64 = 10000;

fn main() {
    let mut monkeys = include_str!("../input.txt").split("Monkey")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut monkey = Monkey{
                item_worry: vec![],
                operation: Box::new(|x| x),
                test: 0,
                action_on_fail: 0,
                action_on_success: 0,
                inspection_count: 0,
            };
            monkey.parse_monkey(x);
            monkey
        })
        .collect::<Vec<Monkey>>();
    
    let factor = monkeys.iter().map(|x| x.test).product::<u64>();

    for _ in 0..NUMBER_OF_ROUNDS {
        for x in 0..monkeys.len() {
            monkeys[x].inspect(factor);
            let throw = monkeys[x].throw();
            for (action, item_worry) in throw {
                monkeys[action].add_worry(item_worry);
            }
        };
    }

    let mut res = monkeys.iter()
                       .map(|x| x.inspection_count)
                       .collect::<Vec<u64>>();
    res.sort();

    println!("{:?}",res);

    println!("{:?}",res[res.len()-1]*res[res.len()-2]);


}


struct Monkey <'a>{
    item_worry: Vec<u64>,
    operation:Box<dyn Fn(u64) -> u64 + 'a>,
    test: u64,
    action_on_fail: u64,
    action_on_success: u64,
    inspection_count: u64,
}

impl PartialEq for Monkey<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.item_worry == other.item_worry
            && self.test == other.test
            && self.action_on_fail == other.action_on_fail
            && self.action_on_success == other.action_on_success
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl fmt::Debug for Monkey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("item_worry", &self.item_worry)
            .field("test", &self.test)
            .field("action_on_fail", &self.action_on_fail)
            .field("action_on_success", &self.action_on_success)
            .finish()
    }
}

impl <'a>Monkey<'a>{
    fn parse_monkey(&mut self,input:&'a str) {

        let mut lines = input.lines();
        lines.next();

        self.item_worry = lines.next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let operation = lines.next()
                    .unwrap()
                    .split("old")
                    .nth(1)
                    .unwrap()
                    .trim();
        
        self.test = lines.next()
                        .unwrap()
                        .split(" ")
                        .last()
                        .unwrap()
                        .trim()
                        .parse::<u64>()
                        .unwrap();
         
         self.operation = match operation.chars().next().unwrap() {
            '*' => {
                if operation.len() == 1 {
                    Box::new(move |x| (x*x))
                } else {
                    let n = operation[2..].parse::<u64>().unwrap();
                    Box::new(move |x| (x*n))
                }

            },
            '+' => 
                {
                    if let Some(n) = operation[2..].parse::<u64>().ok() {
                        Box::new(move |x| x+n)
                    }else{
                        panic!("Invalid operation");
                    }
                }

            _ => panic!("Invalid operation"),
        };          

            
        self.action_on_success = lines.next()
                        .unwrap()
                        .split(" ")
                        .last()
                        .unwrap()
                        .trim()
                        .parse::<u64>()
                        .unwrap();

        self.action_on_fail = lines.next()
                        .unwrap()
                        .split(" ")
                        .last()
                        .unwrap()
                        .trim()
                        .parse::<u64>()
                        .unwrap();
    }

    // create new inspect method
    fn inspect(& mut self,factor:u64) {
        self.item_worry.iter_mut().for_each(|x| {
            *x = (self.operation)(*x);
            *x = *x % factor;
            self.inspection_count += 1;
        });
    }

    fn throw(& mut self) -> Vec<(usize,u64)> {
        let mut result = vec![];
        while !self.item_worry.is_empty() {
            let item_worry = self.item_worry.remove(0);
            if item_worry % self.test == 0 {
                result.push((self.action_on_success as usize,item_worry))
            }else{
                result.push((self.action_on_fail as usize,item_worry))
            }
        }
        result
    }

    fn add_worry(&mut self, item_worry:u64){
        self.item_worry.push(item_worry);
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkeys() {
        let expected_monkeys = vec![
            Monkey {
                item_worry: vec![79, 98],
                operation: Box::new(|x| x * 19),
                test: 23,
                action_on_fail: 2,
                action_on_success: 3,
                inspection_count: 0,
            },
            Monkey {
                item_worry: vec![54, 65, 75, 74],
                operation: Box::new(|x| x + 1),
                test: 19,
                action_on_fail: 2,
                action_on_success: 0,
                inspection_count: 0,
            },
            Monkey {
                item_worry: vec![79, 60, 97],
                operation: Box::new(|x| x * x),
                test: 13,
                action_on_fail: 1,
                action_on_success: 3,
                inspection_count: 0,
            },
            Monkey {
                item_worry: vec![74],
                operation: Box::new(|x| x + 3),
                test: 17,
                action_on_fail: 0,
                action_on_success: 1,
                inspection_count:0,
            },
        ];
        
    }

    #[test]
    fn test_parse_monkey() {
        let expected_monkey = Monkey {
            item_worry: vec![79, 98],
            operation: Box::new(|x| x * 19),
            test: 23,
            action_on_fail: 2,
            action_on_success: 3,
            inspection_count: 0,
        };
        
        let mut monkey = Monkey{
            item_worry: vec![],
            operation: Box::new(|x| x),
            test: 0,
            action_on_fail: 0,
            action_on_success: 0,
            inspection_count: 0,
        };

        monkey.parse_monkey("Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3
      ");

      println!("{:?}",monkey);
      
    }
}
 


