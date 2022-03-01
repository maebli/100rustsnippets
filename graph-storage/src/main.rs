fn main() {
    // Implementation of example
    // on page 10 of https://web.stanford.edu/class/cs97si/06-basic-graph-algorithms.pdf
    const M: usize = 8;
    const N: usize = 5;

    let mut e = [(usize::MAX, usize::MAX); M];
    let mut le = [usize::MAX; N];

    let edges: [(usize, usize); 8] = [
        (0, 1),
        (1, 2),
        (0, 2),
        (0, 4),
        (3, 2),
        (2, 1),
        (3, 1),
        (1, 4),
    ];

    let mut edge_id = 0;

    for edge in edges {
        println!("Adding Edge: {} -> {}", edge.0, edge.1);

        let from = edge.0;
        let to = edge.1;

        e[edge_id].0 = to;
        e[edge_id].1 = le[from];
        le[from] = edge_id;

        edge_id += 1;
    }

    println!("Edges = {:?}", e);
    println!("Last Edges = {:?}", le);

    // iterate over edges starting from 0
    let mut id = le[0];
    while e[id].1 != usize::MAX {
        print!(" 0 -> {}", e[id].0);
        id = e[id].1;
    }

    print!(" 0 -> {}", e[id].0);
}
