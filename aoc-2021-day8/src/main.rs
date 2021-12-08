use std::borrow::BorrowMut;
use std::io::BufRead;
use std::ops::Range;
use byte_set::ByteSet;
use itertools::Itertools;

fn main() {

    let input:Vec<(&str,&str)> = INPUT.lines()
        .map(|x|{
            let y = x.split_once(" | ").unwrap();
            (y.0,y.1)})
        .collect();

    let x= input.iter()
        .map(|x|{x.1.split_whitespace()
            .map(|x|{x.chars().count()})
            .filter(|x|{[2,3,4,7].contains(x)})
            .count()})
        .fold(0,|acc,inst|acc+inst);

    println!("sol1:{:?}",x);

    let mut sol2 = 0;

    for x in input {

        let k:Vec<ByteSet> = x.0.split_whitespace()
            .map(|x|x.chars()
                          .sorted()
                          .collect::<String>()
            ).sorted_by(|a,b|{Ord::cmp(&a.chars().count() ,&b.chars().count())})
            .map(|x|{ByteSet::from(&*x)})
            .collect();

        let one = 0;
        let four = 2;
        let seven = 1;
        let eight = 9;


        let three = k.iter().position(|x|x == {
                k.iter()
                    .filter(|x| x.len() == 5 && k.get(one).unwrap().is_subset(x))
                    .next()
                    .unwrap()
            }
        ).unwrap();

        let nine = k.iter().position(|x|x == {
                k.iter()
                    .filter(|x| x.len() == 6 && k.get(three).unwrap().is_subset(x))
                    .next()
                    .unwrap()
            }
        ).unwrap();

        let six = k.iter().position(|x|x == {
                k.iter()
                    .filter(|x| x.len() == 6 && !k.get(nine).unwrap().is_subset(x))
                    .next()
                    .unwrap()
            }
        ).unwrap();


        let two = k.iter().position(|x|x == {
                 k.iter().enumerate()
                     .filter(|x|{x.0 != three})
                     .map(|x| x.1)
                     .filter(|x| x.len() == 5 && !k.get(six).unwrap().is_subset(x))
                     .next()
                     .unwrap()
            }
        ).unwrap();


        let zero = k.iter().position(|x|x == {
            k.iter().enumerate()
                .filter(|x|{x.0 != six && x.0 !=nine})
                .map(|x| x.1)
                .filter(|x| x.len() == 6)
                .next()
                .unwrap()
        }
        ).unwrap();

        let a = [zero,one,two,three,four,six,seven,eight,nine];
        let five = (0..10).into_iter().filter(|x|{!a.contains(x)}).collect::<Vec<usize>>();
        let five = *five.iter().next().unwrap();

        let sorted = vec![
            k.get(zero).unwrap(),
            k.get(one).unwrap(),
            k.get(two).unwrap(),
            k.get(three).unwrap(),
            k.get(four).unwrap(),
            k.get(five).unwrap(),
            k.get(six).unwrap(),
            k.get(seven).unwrap(),
            k.get(eight).unwrap(),
            k.get(nine).unwrap(),];

        let mut k:Vec<ByteSet> = x.1.split_whitespace()
            .map(|x|x.chars()
                .sorted()
                .collect::<String>()
            )
            .map(|x|{ByteSet::from(&*x)})
            .collect();

        let mut x = 0;

        for i in k {
            x+=sorted.iter().position(|x|i.as_bytes() == x.as_bytes()).unwrap();
            x*=10;
        }
        x/=10;

        println!("{}",x);
        sol2+= x;

    }
    println!("{}",sol2);


}



const INPUT:&str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";