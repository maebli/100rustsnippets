use std::{vec, collections::HashSet};

fn main() {
    let x = beacons_in_field_and_row(include_str!("../input.txt"), 2000000);
    println!("{:?}",x);

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}
struct Probe {
    beacon: Point,
    sensor: Point,
}

fn parse_line(line: &str) -> Probe {
    let coords: Vec<i64> = line
        .split(|c| c == ',' || c == ':' || c == '=')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let sensor = Point { x: coords[0], y: coords[1] };
    let beacon = Point { x: coords[2], y: coords[3] };

    Probe { beacon, sensor }
}


fn get_points_in_row(probe: &Probe, row: i64) -> Vec<Point> {
    let mut points = vec![];

    let sensor = probe.sensor;
    let beacon = probe.beacon;

    let max_distance = ((sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs() - (sensor.y - row).abs()).max(0);

    for i in 0..=max_distance {
        points.push(Point { x: sensor.x + i, y: row });
        points.push(Point { x: sensor.x - i, y: row });
    }

    points
}

fn beacons_in_field_and_row(input: &str, row: i64) -> usize {
    let probes: Vec<Probe> = input.lines().map(parse_line).collect();

    let mut not_possible_positions: HashSet<Point> = HashSet::new();
    let mut beacon_positions: HashSet<Point> = HashSet::new();

    for probe in &probes {
        let points_in_row = get_points_in_row(probe, row);
        not_possible_positions.extend(points_in_row);

        if probe.beacon.y == row {
            beacon_positions.insert(probe.beacon);
        }
    }

    not_possible_positions.difference(&beacon_positions).count()
}


#[cfg(test)]
mod tests {
    use crate::beacons_in_field_and_row;

    #[test]
    fn count_beaconless_positions() {
        let out = beacons_in_field_and_row(include_str!("../test_input.txt"), 10);
        assert_eq!(out, 26);
    }   
}

