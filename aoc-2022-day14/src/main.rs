fn main() {

    let mut field = include_str!("../input.txt")
                                                .lines()
                                                .fold(vec![] , |mut acc,line| {
                                                    let line = line.split("->").collect::<Vec<&str>>();
                                                    for (a,b) in line.iter().zip(line.iter().skip(1)) {

                                                        let f = |x:&str| x.trim()
                                                                                            .split(",")
                                                                                            .map(|x| x.parse::<i32>().unwrap())
                                                                                            .collect::<Vec<i32>>();
                                                        acc.push(vec![f(a),f(b)]);

                                                    }
        acc
    });


    let lowest_point = find_lowest_point(&field);

    // uncomment this for solution 1
    field.push(vec!(vec![0,lowest_point+2],vec![1000,lowest_point+2]));

    dbg!(lowest_point);

    let mut point = vec![500,0];
    let mut sand_units = 0;

    while point[1] <= lowest_point+2 {
        
        point[1] += 1;

        if !check_if_point_collides_with_vectors(&point,&field) {
            continue;
        }

        point[0] -= 1;

        if !check_if_point_collides_with_vectors(&point,&field) {
            continue;
        }

        point[0] += 2;

        if !check_if_point_collides_with_vectors(&point,&field) {
            continue;
        }

        if point[1] == 1 {
            println!("{} {}" , point[0],point[1]);
            break;
        }

        point[0] -= 1;
        point[1] -= 1;

        field.push(vec![vec![point[0],point[1]],vec![point[0],point[1]]]);
        point = vec![500,0];
        sand_units += 1;

    }

    dbg!(sand_units+1);




}

fn check_if_point_collides_with_vectors(point:&Vec<i32>,vectors:&Vec<Vec<Vec<i32>>>) -> bool {

    let (p_x,p_y) = (point[0],point[1]);

    vectors.iter().any(|vector| {
        let a_x = vector[0][0].max(vector[1][0]);
        let b_x = vector[0][0].min(vector[1][0]);
        let a_y = vector[0][1].max(vector[1][1]);
        let b_y = vector[0][1].min(vector[1][1]);
        p_x <= a_x && p_x >= b_x && p_y <= a_y && p_y >= b_y
    })
}


fn find_lowest_point(vectors:&Vec<Vec<Vec<i32>>>) -> i32 {

    vectors.concat()
            .iter()
            .map(|x| x[1])
            .max()
            .unwrap()

}

