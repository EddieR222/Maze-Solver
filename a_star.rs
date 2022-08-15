use std::collections::{HashMap};
use irrgarten::*;
use rayon::prelude::*;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
pub struct Node {
    row: isize, 
    col: isize,
    g_score: isize,
    h_score: isize,
    f_score: isize,
    parent: isize,
}

impl Node {
    pub fn new( row: isize, col: isize) -> Node {
        Node {row: row, col: col, g_score: 0, h_score: 0 , f_score: 0 , parent: 0}
        
    }

    pub fn calc_and_update_scores(&mut self, end_row: isize, end_col: isize) {
        // First we calulate the width and height of a triangle connecting the two points 
        let mut width:isize  = (end_row- self.row).abs();
        let mut height = (end_col - self.col).abs();
        let mut h_score:isize = 0;
        // Then we add 14 (a nice round number for hypothnusus) until either width is 0 or height is 0 
        while (width > 0) && (height > 0) {
            h_score += 14;
            width -= 1;
            height -= 1;
        }
        // Now we add either nonzero width or height as a multiple of 10
        if width > 0 {
            h_score += 10 * width;
        } else if height > 0 {
            h_score += 10 * height;
        }
        // Now we update the f-score
        self.f_score = h_score + self.g_score;
    } 
    
    pub fn compare_pos(&self, other: &Node) -> bool {
        if (self.row == other.row) && (self.col == other.col) {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_successors(maze: &Maze, parent: &Node) -> Vec<Node> {
        let mut successors: Vec<Node> = Vec::new();
        let mut row = parent.row;
        let mut col = parent.col;
        let parent_str = Node::valid_node(row, col, maze).unwrap();
        // REMEMBER, ROW IS Y and COL is X
        let pointer = parent.row_col_to_pointer(maze.width as isize);
        let parent_g = parent.g_score;


        // North Direction
        row = parent.row - 1;
        col = parent.col;
        let node: Option<Node> = Node::valid_node(row, col, maze);
        if !node.is_none() {
            let mut node = node.unwrap();
            node.parent = pointer;
            node.g_score = parent_g + 10;
            successors.push(node);
        }

        // North East Direction 
        row = parent.row - 1;
        col = parent.col + 1;
        let node: Option<Node> = Node::valid_node(row, col, maze);
        if !node.is_none() {
            let mut node = node.unwrap();
            node.parent = pointer;
            node.g_score = parent_g + 14;
            successors.push(node);
        }

        // East Direction
        row = parent.row;
        col = parent.col + 1;
        let node: Option<Node> = Node::valid_node(row, col, maze);
        if !node.is_none() {
            let mut node = node.unwrap();
            node.parent = pointer;
            node.g_score = parent_g + 10;
            successors.push(node);
        }

        // South East Direction
        row = parent.row + 1;
        col = parent.col + 1;
        let node: Option<Node> = Node::valid_node(row, col, maze);
        if !node.is_none() {
            let mut node = node.unwrap();
            node.parent = pointer;
            node.g_score = parent_g + 14;
            successors.push(node);
        }

        // South Direction
        row = parent.row + 1;
        col = parent.col;
        let node: Option<Node> = Node::valid_node(row, col, maze);
        if !node.is_none() {
            let mut node = node.unwrap();
            node.parent = pointer;
            node.g_score = parent_g + 10;
            successors.push(node);
        }

        // South West Direction
        row = parent.row + 1;
        col = parent.col - 1;
        let node: Option<Node> = Node::valid_node(row, col, maze);
        if !node.is_none() {
            let mut node = node.unwrap();
            node.parent = pointer;
            node.g_score = parent_g + 14;
            successors.push(node);
        }

        // West Direction
        row = parent.row;
        col = parent.col - 1;
        let node: Option<Node> = Node::valid_node(row, col, maze);
        if !node.is_none() {
            let mut node = node.unwrap();
            node.parent = pointer;
            node.g_score = parent_g + 10;
            successors.push(node);
        }

        // North West Direction
        row = parent.row - 1;
        col = parent.col - 1;
        let node: Option<Node> = Node::valid_node(row, col, maze);
        if !node.is_none() {
            let mut node = node.unwrap();
            node.parent = pointer;
            node.g_score = parent_g + 14;
            successors.push(node);
        }

        return successors;
    }

    pub fn valid_node(row: isize, col: isize, maze: &Maze) -> Option<Node> {
        if point_inbounds(maze, row, col) {
            if  maze[row as usize][col as usize] != 1 {
            return Some(Node::new(row, col));
            } 
            return None;
        }
        None

    }

    pub fn A_star(maze: &Maze, start_row: isize, start_col: isize, end_row: isize, end_col: isize) -> Vec<(usize, usize)> {
    // Create open and closed lists
    let mut open: Vec<Node> = Vec::new();
    let mut closed: Vec<Node> = Vec::new();
  

    // Now we create the node for the starting point and we add it to the open list
    let start_node = Node::new(start_row, start_col);
    let end_pointer = start_node.row_col_to_pointer(maze.width as isize);
    open.push(start_node.clone());

    // Variables we will need as well, including successors vector
    let mut q: Node;
    let mut successors: Vec<Node> = Vec::new();
    

    while !open.is_empty() {
        // First we grab the lowest f score node from the open list
        let mut least_f_score: isize = isize::MAX;
        let mut least_node: Node = Node::new(0, 0);
        let least_node = open.par_iter().min_by(|node1, node2| node1.f_score.cmp(&node2.f_score));
    
        q = *least_node.unwrap();
        open.retain(|n:& Node| *n != q);
       
       
    
        successors = Node::get_successors(&maze, &q);
    
        'outer: for mut successor in successors.iter_mut() {
            // Check to see if successor is the target point we want to reach
            // if it is, then we stop the search and output our result
            if (successor.row == end_row) && (successor.col == end_col) {
                closed.push(q);
                let path = trace_path(successor, end_pointer, maze.width as isize, closed);
                return path;
            }

            // Calculate the g, h, and f scores for this successor
            successor.calc_and_update_scores( end_row, end_col);

            // Check to see if any node in the open list has the same poition as the successor
            // If it does, meaning they were the same point, and the node has a smaller f score than the successor
            // we skip the successor
            if open.par_iter().any(|node| successor.compare_pos(&node) && node.f_score < successor.f_score) {
                continue 'outer;
            }
           
            // Check to see if any node in the closed list has the same poition as the successor
            // If it does, meaning they were the same point, and the node has a smaller f score than the successor
            // we skip the successor otherwise we add the node to the open list
            if closed.par_iter().any(|node| successor.compare_pos(&node) && node.f_score < successor.f_score) {
                continue 'outer;
            }
            
            
            open.push(*successor);
        
        }

        closed.push(q);
    }

    return Vec::new();

}


    pub fn row_col_to_pointer(&self, width: isize) -> isize {
        (self.row*width + self.col)
    }

    pub fn pointer_to_row_col( pointer: isize, width: isize) -> (usize, usize) {
        let col = pointer%width;
        let row = (pointer as f32)/(width as f32).floor();
        return (row as usize, col as usize);
    }

}



pub fn point_inbounds(maze: &Maze, row: isize, col: isize) -> bool {
    let maze_width: isize = (maze.width) as isize;
    let maze_height: isize = (maze.height) as isize;
    if (row < 0) | (row > maze_width) {
        return false;
    }
    if (col < 0) | (col > maze_height) {
        return false;
    }
    // if we reach here, the row and col provided is inbounds of maze
    return true;
}

pub fn trace_path(begin: &Node, end_pointer: isize, width: isize, closed: Vec<Node>) -> Vec<(usize, usize)> {
    let mut path:Vec<(usize, usize)> = Vec::new();

    let mut curr_node = begin;
    let target_coords = (curr_node.row as usize, curr_node.col as usize);
    path.push(target_coords);
    while curr_node.parent != end_pointer {
        let coordinates = Node::pointer_to_row_col(curr_node.parent, width);
        path.push(coordinates); 
        curr_node = closed.par_iter().find_any(|node| curr_node.parent == node.row_col_to_pointer(width)).unwrap();
    }
    let start_coords = Node::pointer_to_row_col(end_pointer, width);
    path.push(start_coords);
    return path;
}

