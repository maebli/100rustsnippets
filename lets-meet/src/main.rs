use geocoding::{GeoAdmin,Forward,Point};

fn main() {

    let addresses= vec!["Locarno","Lugano","Altdorf"];

    let geoadmin = GeoAdmin::new();

    let weight = 1.0/(addresses.len() as f64);
    let mut center = addresses.iter()
    .map(|address|  {
        *geoadmin.forward(address).unwrap().get(0).unwrap()
    }).fold(Point::new(0.0,0.0), |mut acc, x| {
        acc+=x;
        acc
    });
    center=center*weight;

    println!("Center: {:?}",center);
    println!("https://www.google.com/maps/search/{:?},{:?}", center.y(),center.x());
    
}