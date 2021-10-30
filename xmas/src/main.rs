
fn main() {

    const PLACINGS:&[&str]=&["first","second","third","fourth","fifth","sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];
    const NUMBERS:&[&str]=&["One","Two","Three","Four","Five","Six","Seven","Eight","Nine","Ten","Eleven","Twelve"];
    const THINGS:&[&str]=&["Partridge in a pear tree ","Turtle doves ","French hens",
        "Collie birds","Golden rings","Geese a-laying","Swans a-swimming ","Maids a-milking",
        "Pipers piping ","Drummers drumming ","Lords a-leaping ","Ladies dancing "];

    for num in 1..12{
        println!("On the {} day of Christmas my true love gave to me",PLACINGS[num]);
        for x in (1..(num)).rev() {
            println!("{} {}",NUMBERS[x],THINGS[x]);
        }

        match num {
            1 => print!("A "),
            _ => print!("And a ")
        }
        println!("{}\n",THINGS[0]);
    }
}
