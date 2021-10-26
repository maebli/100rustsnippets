use std::io;

fn main() {

    let mut temp = String::new();

    println!("enter °C value to be converted to Fahrenheit:");

    io::stdin().read_line(& mut temp).expect("oops");

    let temp:f32 = temp.trim().parse().expect("failed to parse");

    println!("{}°C =  {} Fahrenheit",temp,celcius_to_fahrenheit(temp));

}

fn celcius_to_fahrenheit(celclius: f32) -> f32 {
    celclius*1.8+32.0
}
