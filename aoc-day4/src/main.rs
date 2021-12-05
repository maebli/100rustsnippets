use std::process::exit;

fn main() {

    let bingo_numbers:Vec<u32> = BINGO_NUMBERS
        .split(",")
        .map(|x|{x.parse().unwrap()})
        .collect();

    let bingo_boards:Vec<&str> = BINGO_BOARDS
        .split("\n\n")
        .collect();

    let mut boards:Vec<BingoBoard> = vec![];

    for bingo_board in bingo_boards {

        let x:Vec<u32> = bingo_board
            .split_whitespace()
            .map(|x|{String::from(x)})
            .filter(|x|{x != ""})
            .map(|x|{x.parse().unwrap()})
            .collect();

        let b = BingoBoard::from(x);
        boards.push(b);
    }

    for x in bingo_numbers {
        let mut board_num:u32 = 0;
        for b in boards.iter_mut() {
            board_num += 1;

            b.update(x);

            if b.we_have_a_winner() {
                println!("Last called Number = {}",x);
                println!("we have a winner, board #{}, solution =  {}*{} = {}",board_num,
                    b.sum_of_uncalled(),x,b.sum_of_uncalled()*x);
                    exit(0);
            }
        }
    }
}



struct BingoBoard{
    fields:Vec<BingoField>
}

#[derive(Clone)]
struct BingoField{
    was_called:bool,
    number:u32
}



impl BingoBoard {

    fn we_have_a_winner(&self) -> bool {

        let mut is_winner = true;

        for j in 0..5 {
            for i in 0..5 {
                is_winner &= self.fields
                    .get(j*5 + i)
                    .unwrap()
                    .was_called;
            }
            if is_winner {
                return true
            }
            is_winner = true;
        }

        for j in 0..5 {
            for i in 0..5 {
                is_winner &= self.fields
                    .get(j + i*5)
                    .unwrap()
                    .was_called;
            }

            if is_winner {
                return true
            }

            is_winner = true;
        }

        false
    }

    fn update(&mut self,num:u32) ->bool{
        for b in &mut self.fields {
            if b.number == num {
                b.was_called = true;
                return true;
            }
        }
        false
    }

    fn from (v:Vec<u32>) -> BingoBoard {
        let mut b = BingoBoard{ fields: vec![] };
        for x in v {
            let f = BingoField{ was_called: false, number: x };
            b.push(f);
        }
        b
    }

    fn push (&mut self,f:BingoField){
        self.fields.push(f);
    }

    fn sum_of_uncalled(&self) -> u32{
        self.fields.iter()
            .filter(|x|{!x.was_called })
            .fold(0,|acc,inst|{acc + inst.number})
    }
}


const BINGO_NUMBERS:&str ="7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
const BINGO_BOARDS:&str ="22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";