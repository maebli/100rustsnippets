use itertools::Itertools;

fn main(){

}

fn count_duplicates(text: &str) -> u32 {

    if text.len() == 0 {
        return 0
    };

    let x = text.to_ascii_lowercase().chars().sorted().collect_vec();

    let mut last = '?';
    let mut consecutive_chars = 0;
    let mut total_doubles = 0;
    let very_last = *x.last().unwrap();
    println!("{:?}",x);

    for current in x {
        if last == very_last {
            total_doubles += 1;
        }else if current == last{
            consecutive_chars += 1;
        }else{
            if consecutive_chars >= 1 {
                total_doubles += 1;
            }
            consecutive_chars = 0;
        }
        last = current;
    }

    total_doubles

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("wP66iqNlA4pctFR2ge5dal66Et2opctMg4U1aw6BYJedsDjGNmYkEQzzjKxaMep6vcILWCI2p9YfwWVPDX"), 23);
    }
}
