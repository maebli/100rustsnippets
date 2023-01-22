fn main() {

    let input = include_str!("../input.txt").split("\n\n").enumerate();

    let k:usize = input.map(|(i, group)| {
        let group_number = i+1;
        let left = group.split("\n").next().unwrap();
        let right = group.split("\n").last().unwrap();
        (group_number, compare(left, right))
    }).filter(|(_, x)| *x)
        .map(|(i, _)| i)
        .sum();

   println!("{}", k);

   let mut x = include_str!("../input.txt").lines().filter(|x| !x.eq(&"")).map(str::trim).collect::<Vec<&str>>();
   x.append(&mut vec!["[[2]]"]);
   x.append(&mut vec!["[[6]]"]);

   let mut x = x.iter()
                                            .map(|x| parse_list(x))
                                            .map(|x| x.ok().unwrap().0)
                                            .collect::<Vec<List>>();
   x.sort();

   let a = parse_list("[[2]]").ok().unwrap().0;
   let b = parse_list("[[6]]").ok().unwrap().0;

   let a = x.iter().position(|x| x.eq(&a)).unwrap();
   let b = x.iter().position(|x| x.eq(&b)).unwrap();

   println!("{}", (a+1)*(b+1));


}

#[derive(Debug,Eq,Ord)]
struct List {
    items: Vec<Element>,
}

#[derive(Debug,Eq,Ord)]
enum Element {
    Integer(i64),
    List(Box<List>),
}
impl PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        match (self, other) {
            (&Element::Integer(ref a), &Element::Integer(ref b)) => a == b,
            (&Element::List(ref a), &Element::List(ref b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &List) -> bool {
        self.items == other.items
    }
}


impl PartialOrd for List {
    fn partial_cmp(&self, other: &List) -> Option<std::cmp::Ordering> {
        self.items.partial_cmp(&other.items)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Element) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (&Element::Integer(ref a), &Element::Integer(ref b)) => a.partial_cmp(b),
            (&Element::List(ref a), &Element::List(ref b)) => a.partial_cmp(b),
            (&Element::List(ref a), &Element::Integer(ref b)) => a.partial_cmp(&Box::new(List { items: vec![Element::Integer(*b)] })),
            (&Element::Integer(ref a), &Element::List(ref b)) => Box::new(List { items: vec![Element::Integer(*a)] }).partial_cmp(b),
        }
    }
}

fn parse_list(input: &str) -> Result<(List, &str), &str> {
    let mut items = vec![];
    let mut input = input.trim_start();

    if !input.starts_with("[") {
        return Err("Expected '[' at start of list");
    }
    input = &input[1..];

    while !input.starts_with("]") {
        let (item, rest) = parse_element(input)?;
        items.push(item);
        input = rest.trim_start();

        if input.starts_with(",") {
            input = &input[1..];
        }
    }
    input = &input[1..];

    Ok((List { items }, input))
}

fn parse_element(input: &str) -> Result<(Element, &str), &str> {
    if input.starts_with("[") {
        parse_list(input).map(|(list, rest)| (Element::List(Box::new(list)), rest))
    } else {
        parse_integer(input)
    }
}

fn parse_integer(input: &str) -> Result<(Element, &str), &str> {
    let mut end = 0;
    while end < input.len() && input.as_bytes()[end] >= b'0' && input.as_bytes()[end] <= b'9' {
        end += 1;
    }
    let (integer_str, rest) = input.split_at(end);
    let integer = integer_str.parse::<i64>().map_err(|_| "Expected integer")?;
    Ok((Element::Integer(integer), rest))
}

fn compare(left: &str, right: &str) -> bool {

    let left = parse_list(left);
    let right = parse_list(right);
    
    match (left, right) {
        (Ok((left, _)), Ok((right, _))) => left < right,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        test("[1,1,3,1,1]", "[1,1,5,1,1]", true);
    }

    #[test]
    fn test2() {
        test("[[1],[2,3,4]]", "[[1],4]", true);
    }

    #[test]
    fn test3() {
        test("[9]", "[[8,7,6]]", false);
    }

    #[test]
    fn test4() {
        test("[[4,4],4,4]", "[[4,4],4,4,4]", true);
    }

    #[test]
    fn test5() {
        test("[7,7,7,7]", "[7,7,7]", false);
    }

    #[test]
    fn test6() {
        test("[]", "[3]", true);
    }

    #[test]
    fn test7() {
        test("[[[]]]", "[[]]", false);
    }

    #[test]
    fn test8() {
        test("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", false);
    }

    fn test(input: &str, output: &str, expected_result: bool) {
        let input = input.to_string();
        let output = output.to_string();
        let result = compare(&input, &output);
        assert_eq!(result, expected_result, "input: {}, Output: {}", input, output);
    }
    
}