fn main() {

}

fn sort(unsorted:&[i32]) -> Vec<i32>{
    let mut sorted:Vec<i32> = Vec::new();

    if unsorted.len() == 1 {
       sorted.push(unsorted[0]);
    }else {
        sorted = sort(&unsorted[1..]);

        for x in 0..sorted.len() {
            if unsorted[0] < sorted.as_slice()[x] {
                sorted.insert(x, unsorted[0]);
                break;
            }else if x == sorted.len()-1{
                sorted.push(unsorted[0]);
            }
        }
    }

    sorted
}

#[test]
fn test_sort(){
    let unsorted = vec![9, 8, 3, 7, 28, -4, -99999];
    let sorted = vec![-99999,-4,3,7,8,9,28];
    assert_eq!(sort(&unsorted),sorted);
}