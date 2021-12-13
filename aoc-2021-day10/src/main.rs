
fn main() {

    let mut score = (0,0,0,0);
    let mut last = 0u8;
    let sol1 = INPUT.lines().map(|x|{

        let index = x.as_bytes()
            .iter()
            .take_while(|x|{last = **x;consume(**x, &mut score)})
            .count();

        match last {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _    => 0
        }

    }).fold(0,|acc,inst|{acc+inst});

    println!("sol1:{}",sol1);

}

fn consume(b:u8, score:& mut (i32, i32, i32, i32)) -> bool{
    match b {
        b'('  =>  score.0 += 1,
        b')'  =>  score.0 -= 1,
        b'['  =>  score.1 += 1,
        b']'  =>  score.1 -= 1,
        b'{'  =>  score.2 += 1,
        b'}'  =>  score.2 -= 1,
        b'<'  =>  score.3 += 1,
        b'>'  =>  score.3 -= 1,
        _     =>  ()
    }
    return score.0 >=  0 && score.0 >=  0 && score.0 >=  0 && score.0 >= 0
}


const INPUT:&str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";