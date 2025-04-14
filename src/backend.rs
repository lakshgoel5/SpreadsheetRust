// getting_things_updated function here
// all operations to be done here
use crate::types::{Coordinates};
use crate::graph::Node;
// functions to be added :
// 1. generate_grid
// 2. add_edges
// 3. break_edges
// 4. 

// static mut IS_DISABLED: bool = false;
// debug // change this
// static mut GRID: Option<Vec<Vec<types::Node>>> = None;

// fn generate_grid(r: usize, c: usize) {
//     unsafe {
//         GRID = Some(vec![vec![0; c + 1]; r + 1]);
//     }
// }

pub fn generate_grid(r: usize, c: usize) -> Vec<Vec<Node>> {
    (0..r)
        .map(|i| {
            (0..c)
                .map(|j| Node {
                    node_value: 0,
                    function: -1,
                    value1: Coordinates { row: -1, col: -1 },
                    value2: Coordinates { row: -1, col: -1 },
                    position: Coordinates { row: i as i32, col: j as i32 },
                    valid: true,
                    dependents: Vec::new(),
                })
                .collect()
        })
        .collect()
}

// wrt new node -- inward
pub fn add_edges_inward(
    graph: &mut Vec<Vec<Node>>,
    value1: Coordinates,
    value2: Coordinates,
    new_node: Coordinates,
) {
    // add in dependency list of vectors

    // range based functions
    for i in value1.row as usize ..= value2.row as usize {
        for j in value1.col as usize ..= value2.col as usize {
            graph[i][j].add_dep(new_node);
        }
    }
}

// will fill when structure is more clear
// debug -- see the old dependecies where they are stored in the graph when fully made
// wrt new node -- inward
pub fn break_edges_inward(
    graph: &mut Vec<Vec<Node>>,
    value1: Coordinates,
    value2: Coordinates,
    new_node: Coordinates,
){

    // remove from dependency list of parent(old dependecies)
}

pub fn update_grid(
    graph: &mut Vec<Vec<Node>>,
    value1: Coordinates,
    value2: Coordinates,
    new_node: Coordinates,
) {
    // update the grid with the new node
    // add edges to the graph
    // add_edges_inward(graph, value1, value2, new_node);
}
