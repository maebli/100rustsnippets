fn main() {
    let answer1 = INPUT.lines()
        .filter(|&x| Entry::new(x).check_sled_rental())
        .count();

    let answer2 = INPUT.lines()
        .filter(|&x| Entry::new(x).check_toboggan_rental())
        .count();

    println!("answer1:{} answer2:{}",answer1,answer2);
}

struct Entry {
    left:usize,
    right:usize,
    letter:char,
    password:String,
}

impl Entry{

    fn check_toboggan_rental(&self) -> bool {
        self.password.chars().nth(self.left - 1).unwrap().eq(&self.letter) ^
            self.password.chars().nth(self.right -1 ).unwrap().eq(&self.letter)
    }

    fn check_sled_rental(&self) -> bool {

        let num = self.password
            .chars()
            .filter(|c| c.eq(&self.letter))
            .count();

        num >= self.left && num <= self.right
    }

    fn new(line:&str) -> Entry{

        let mut s:Vec<&str> = line
            .split(' ')
            .collect();

        let range = s
            .remove(0)
            .split_once('-')
            .unwrap();

        let min:usize = range
            .0
            .parse()
            .unwrap();

        let max:usize = range
            .1
            .parse()
            .unwrap();

        let letter = s
            .remove(0)
            .trim_end_matches(":")
            .chars()
            .nth(0)
            .unwrap();

        let password = s.remove(0);

        Entry{ left: min, right: max, letter, password: String::from(password) }
    }
}



const INPUT:&str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";