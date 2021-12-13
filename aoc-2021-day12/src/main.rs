use std::collections::HashMap;
use std::ops::Index;
use petgraph::dot::Dot;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::prelude::{Bfs, Dfs};

fn main() {

    let mut input:Vec<(&str, &str)> = INPUT.lines()
        .map(|x|  x.split_once("-").unwrap())
        .collect();

    let mut graph= Graph::<&str, u32>::new();

    let mut map:HashMap<&str,NodeIndex> = HashMap::new();

    input.iter()
        .for_each(|x|{
            if !map.contains_key(x.0){
                map.insert(x.0,graph.add_node(x.0));
            }
            if !map.contains_key(x.1){
                map.insert(x.1,graph.add_node(x.1));
            }
            graph.add_edge(*map.get(x.0).unwrap(), *map.get(x.1).unwrap(), 1);
        });

    let mut inversed_map:HashMap<usize,&str> = map.iter()
        .map(|x|(x.1.index(),*x.0))
        .collect();

    println!("{}", Dot::new(&graph));


}



const INPUT:&str ="pf-pk
ZQ-iz
iz-NY
ZQ-end
pf-gx
pk-ZQ
ZQ-dc
NY-start
NY-pf
NY-gx
ag-ZQ
pf-start
start-gx
BN-ag
iz-pf
ag-FD
pk-NY
gx-pk
end-BN
ag-pf
iz-pk
pk-ag
iz-end
iz-BN
";