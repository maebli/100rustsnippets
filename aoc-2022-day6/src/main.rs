use std::collections::HashSet;

fn main() {
   let ans1 = get_signal_start_offset(include_str!("../input.txt"),4);
   println!("answer 1: {}", ans1);
   let ans1 = get_signal_start_offset(include_str!("../input.txt"),14);
   println!("answer 1: {}", ans1);
}

fn get_signal_start_offset(signal:&str,length:usize) -> usize {
    signal.as_bytes()
          .windows(length)
          .enumerate()
          .filter(|w| {
                w.1.into_iter().fold(HashSet::new(), |mut acc, c| {
                    acc.insert(*c);
                    acc
                }).len() == length
            }
        ).next()
        .unwrap()
        .0+length
}

/* create tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_signal_start_offset("bvwbjplbgvbhsrlpgdmjqwftvncz",4), 5);
        assert_eq!(get_signal_start_offset("nppdvjthqldpwncqszvftbrmjlhg",4), 6);
        assert_eq!(get_signal_start_offset("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",4), 10);
        assert_eq!(get_signal_start_offset("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4), 11); 
        assert_eq!(get_signal_start_offset("mjqjpqmgbljsphdztnvjfqwrcgsmlb",14), 19);
    }
}