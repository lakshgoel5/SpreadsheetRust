use std::collections::HashMap;

use crate::types::Coordinates;

#[derive(Debug)]
pub struct Node {
    pub function: i32,
    pub value1: Coordinates,
    pub value2: Coordinates,
    pub position: Coordinates,
    pub edges: HashMap<(usize, usize), ()>, // simulate HashTable: mapping Coordinates to empty value
    pub valid: bool,
}

#[derive(Debug)]
pub struct Graph {
    pub matrix: Vec<Vec<Option<Node>>>,
    pub rows: usize,
    pub cols: usize,
}

impl Graph {
    pub fn new(rows: usize, cols: usize) -> Option<Self> {
        if rows > 1000 || cols > 18279 {
            return None;
        }
        let mut matrix = vec![vec![None; cols + 1]; rows + 1];
        Some(Self { matrix, rows, cols })
    }

    pub fn add_node(&mut self, function: &i32, value1: &Coordinates, value2: &Coordinates, row: usize, col: usize) {
        if row > self.rows || col > self.cols || row == 0 || col == 0 {
            return;
        }
        if self.matrix[row][col].is_some() {
            return;
        }
        let node = Node {
            function: *function,
            value1: *value1,
            value2: *value2,
            position: Coordinates { row, col },
            edges: HashMap::new(),
            valid: true,
        };
        self.matrix[row][col] = Some(node);
    }

    pub fn add_edge(&mut self, from: &Coordinates, to: &Coordinates) {
        if let Some(node) = self.matrix[from.row][from.col].as_mut() {
            node.edges.insert((to.row, to.col), ());
        }
    }

    pub fn remove_edge(&mut self, from: &Coordinates, to: &Coordinates) {
        if let Some(node) = self.matrix[from.row][from.col].as_mut() {
            node.edges.remove(&(to.row, to.col));
        }
    }

    pub fn has_cycle(&self, start: &Coordinates) -> bool {
        let mut visited = vec![vec![false; self.cols + 1]; self.rows + 1];
        let mut in_stack = vec![vec![false; self.cols + 1]; self.rows + 1];
        self.has_cycle_util(start, &mut visited, &mut in_stack)
    }

    fn has_cycle_util(&self, coord: &Coordinates, visited: &mut Vec<Vec<bool>>, in_stack: &mut Vec<Vec<bool>>) -> bool {
        if visited[coord.row][coord.col] {
            return false;
        }
        visited[coord.row][coord.col] = true;
        in_stack[coord.row][coord.col] = true;

        if let Some(node) = &self.matrix[coord.row][coord.col] {
            for &(r, c) in node.edges.keys() {
                if in_stack[r][c] {
                    return true;
                }
                if !visited[r][c] && self.has_cycle_util(&Coordinates { row: r, col: c }, visited, in_stack) {
                    return true;
                }
            }
        }

        in_stack[coord.row][coord.col] = false;
        false
    }
    pub fn topological_sort(&self, start: &Coordinates) -> Vec<Coordinates> {
        let mut visited = vec![vec![false; self.cols + 1]; self.rows + 1];
        let mut result = Vec::new();
        self.topological_util(start, &mut visited, &mut result);
        result
    }

    fn topological_util(&self, coord: &Coordinates, visited: &mut Vec<Vec<bool>>, result: &mut Vec<Coordinates>) {
        if visited[coord.row][coord.col] {
            return;
        }
        visited[coord.row][coord.col] = true;

        if let Some(node) = &self.matrix[coord.row][coord.col] {
            for &(r, c) in node.edges.keys() {
                self.topological_util(&Coordinates { row: r, col: c }, visited, result);
            }
        }
        result.push(*coord);
    }
}
