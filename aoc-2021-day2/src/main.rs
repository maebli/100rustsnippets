fn main() {

    let parsed_input:Vec<(&str, i32)> = INPUT.lines()
        .map(|x|{x.split_once(" ").unwrap()})
        .map(|x|{(x.0,x.1.parse::<i32>().unwrap())})
        .collect();

    let mut pos = Position{ x: 0, y: 0 };

    pos.y += parsed_input.iter()
        .filter(|x|{x.0 == "up"})
        .fold(0,|acc, c| { acc + c.1 });

    pos.y -= parsed_input.iter()
        .filter(|x|{x.0 == "down"})
        .fold(0,|acc, c| { acc + c.1 });

    pos.x += parsed_input.iter()
        .filter(|x|{x.0 == "forward"})
        .fold(0,|acc, c| { acc + c.1 });

    pos.print();

    let aim_delta:Vec<i32> = parsed_input.iter()
        .map(|x| { match x.0 {
            "up" => -x.1,
            "down" => x.1,
            _ => 0
        }})
        .collect();

    let out:Vec<i32> = parsed_input.iter()
        .map(|x|{ match x.0 {
            "forward" => x.1,
            _ => 0
        }})
        .collect();

     let pos = out.iter()
         .zip(aim_delta.iter())
         .fold((0,Position{x:0,y:0}),|mut acc, x|{
             acc.1.x += x.0;
             acc.1.y += acc.0*x.0;
             acc.0 -= x.1;
             acc
         }).1;

    pos.print();

}

struct Position{
    x:i32,
    y:i32
}

impl Position{
    fn print(&self){
        println!(" x = {} , y = {}, depth = {}, x * depth = {}",self.x,self.y,-self.y,-self.x*self.y);
    }
}


const INPUT:&str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";