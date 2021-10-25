use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0..100);
    let t1 = generate_tree(a);

    println!("digraph G {{");
    print_tree(a,t1);
    println!("}}");
}


fn print_tree(root:i32,tree:(i32,char,i32)){
    println!("\"{}\"->\"{}\"",tree.0,tree.1);
    println!("\"{}\"->\"{}\"",tree.2,tree.1);
    println!("\"{}\"->\"{}\"",tree.1,root);
}

fn generate_tree(seed:i32) -> (i32,char,i32) {

    let mut rng = rand::thread_rng();
    let operators = ['+','-','*'];

    let operator = operators[2];//rng.gen_range(0..operators.len())];

    let mut operator1= 0;
    let mut operator2 = 0;

    match operator {
        '+' => {
            operator1 = rng.gen_range(0..seed);
            operator2 = seed - operator1;
        },
        '-' => {
            operator1 = rng.gen_range(seed..100);
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
    while num % x !=0 {
        x = rng.gen_range(1..((num / 2)+1));
    }
    return x;
}