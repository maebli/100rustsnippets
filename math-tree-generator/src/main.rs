use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("digraph G {{");
    let mut a = rng.gen_range(10..100);
    let mut t = generate_tree(a);

    for x in 0..rng.gen_range(1..3){
        print_tree(a,t,0);
        a = t.2;
        t = generate_tree(a);
    }
    println!("}}");
}


fn print_tree(root:i32,tree:(i32,char,i32),depth:i32){
    let mut operator = String::from(tree.1);
    let mut o1 = String::from(tree.0.to_string());
    let mut o2 = String::from(tree.2.to_string());
    let mut root = String::from(root.to_string());

    for _ in 0..depth {
        operator.push(' ');
        o1.push(' ');
        o2.push(' ');
        o1.push(' ');
        o2.push(' ');
        root.push(' ');
    }

    println!("\"{}\"->\"{}\"",o1,operator);
    println!("\"{}\"->\"{}\"",o2,operator);
    println!("\"{}\"->\"{}\"",operator,root);
}

fn generate_tree(seed:i32) -> (i32,char,i32) {

    let mut rng = rand::thread_rng();
    let operators = ['+','-','*'];

    let operator = operators[rng.gen_range(0..operators.len())];

    let mut operator1= 0;
    let mut operator2 = 0;

    match operator {
        '+' => {
            operator1 = rng.gen_range(2..seed);
            operator2 = seed - operator1;
        },
        '-' => {
            operator1 = rng.gen_range(seed..98);
            operator2 = seed - operator2;
        },
        '*' => {
            operator1=find_a_random_factor(seed);
            operator2=seed/operator1;
        }
        _ => {}
    }

    (operator1,operator,operator2)
}

fn find_a_random_factor(num:i32) -> i32{

    let mut rng = rand::thread_rng();
    let mut x = 100;
    while num % x !=0 && num !=1 {
        x = rng.gen_range(1..((num / 2)+1));
    }
    return x;
}