use solver::{self, solve_spaceman_greedy::*};

fn main() {
    for i in 24..26 {
        if i == 22 {
            continue;
        }
        let filepath = format!("<your_folder>/icfpc2024/py/history/spaceship/{}", i);
        println!("Solve {}: ", i);
        let positions = read_positions(&filepath);
        let solution = solve_greedy(positions);
        match (solution) {
            Some(moves) => {
                print_to_console(&moves);
            },
            None => {
                println!("Failure!");
            }
        }
        //solve_greedy_nearest_x_first(positions);
        //solve_greedy_nearest_y_first(positions);
    }
}
