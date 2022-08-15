
pub mod a_star;
use a_star::*;
use macroquad::{prelude::*};
use irrgarten::*;
extern crate rand;


#[macroquad::main("A-Star")]
async fn main() {
    let width = 701;
    let height = 701;
    request_new_screen_size(2500.0,1200.0);
    let mut maze1 = generate_maze(width, height);
    let points = find_start_and_end_point(&maze1,width,height);
    // println!("Start Point: {:?}", points.0);
    // println!("End Point: {:?}", points.1);
    let path = Node::A_star(&maze1, 1, 1, points.1.0 as isize, points.1.1 as isize);
    // println!("Path: {:?}", path);
    for coords in path.iter() {
        let row = coords.0;
        let col = coords.1;
        maze1[row][col] = 2;
    }
loop {
    

 clear_background(BLACK);

    let width_of_box = screen_width() / width as f32;
    let height_of_box = screen_height() / height as f32;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    for row in maze1.iter() {
        for col in row {
            match col {
                &0 => {
                    draw_rectangle(x, y, width_of_box, height_of_box, WHITE);
                }
                &1 => {
                    draw_rectangle(x, y, width_of_box, height_of_box, BLACK);
                }
                &2 => {
                    draw_rectangle(x, y, width_of_box, height_of_box, GREEN);
                }
                _ => {
                    draw_rectangle(x, y, width_of_box, height_of_box, RED);
                }
            } // end of match statement
            x += width_of_box;
        } // end of col loop
        y += height_of_box;
        x = 0.0;  
    } // end of row loop

    draw_rectangle(points.1.0 as f32 * width_of_box, points.1.1 as f32 * height_of_box, width_of_box, height_of_box, RED);
    next_frame().await;
    
}


    

}


fn generate_maze(width: usize, height: usize) -> Maze {
    let mut rng = rand::thread_rng();
    let maze = Maze::new(width, height).unwrap().generate(&mut rng);
    //  for row in maze.iter() {
    //         println!("{:?}", row);
    //  }
     return maze;
}

fn find_start_and_end_point(maze: &Maze, width: usize, height: usize) -> ((usize, usize), (usize, usize)) {
    let mut start_point = (0, 0);
    for i in 0..5 {
        for j in 0..5 {
            if maze[i][j] == 0 {
                start_point = (i, j);
                break;
            }
        }
    }

    let mut end_point = (0, 0);
    for i in (width - 10)..width {
        for j in (height - 10)..height {
            if maze[i][j] == 0 {
                end_point = (i, j);
            }
        }
    }
    return (start_point, end_point);
}
