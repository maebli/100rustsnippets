fn main() {
    let x = signal_strenth(include_str!("../input.txt"));
    println!("{:?}",x);
    render(include_str!("../input.txt"));
}


fn signal_strenth(input:&str) -> u64 {
    input.lines().fold(vec![1],|mut acc,curr|{
        let last = *acc.last().unwrap();
        acc.push(last);
        if curr[..4] != *"noop"{
            let value = curr.split(" ").last()
                            .unwrap()
                            .parse::<i64>()
                            .ok()
                            .unwrap();
            acc.push(last+value);
        };
        acc
    })
    .iter()
    .enumerate()
    .filter(|i| (i.0+1)%40 == 20)
    .map(|a| ((a.0 as i64)+1)*a.1)
    .sum::<i64>() as u64
}


fn render(input:&str) {
    let x = input.lines().fold(vec![1],|mut acc,curr|{
        let last = *acc.last().unwrap();
        acc.push(last);
        if curr[..4] != *"noop"{
            let value = curr.split(" ").last()
                            .unwrap()
                            .parse::<i64>()
                            .ok()
                            .unwrap();
            acc.push(last+value);
        };
        acc
    })
    .iter()
    .enumerate()
    .map(|a|  {
        if ((a.0%40) as i64-a.1).abs() < 2 {
            return '#'
        }
        '.'
    }
    ).collect::<Vec<_>>();

    let lines = x.chunks(40).map(|a| a.iter().collect::<String>()).collect::<Vec<_>>();
    for line in lines {
        println!("{}",line);
    }
    
}


// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_signal_strenth() {
        assert_eq!(signal_strenth(include_str!("../test_input.txt")),13140);
    }

    #[test]
    fn test_render() {
        render(include_str!("../test_input.txt"));
    }
}