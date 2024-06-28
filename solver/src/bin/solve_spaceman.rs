use solver::{self, solve_spaceman_greedy::*};

fn main() {
    for i in 1..22 {
        if i == 22 {
            continue;
        }
        let problem_filepath = format!("/your_folder/icfpc2024/py/history/spaceship/{}", i);
        println!("Solve {}: ", i);
        let positions = read_positions(&problem_filepath);

        let solution_filepath = format!("/your_folder/icfpc2024/py/solutions/spaceship/{}", i);
        let best_solution = read_from_file(&solution_filepath);
        
        let mut solutions = Vec::new();
        solutions.push(solve_greedy(positions.clone()));
        solutions.push(solve_greedy_nearest_x_first(positions.clone()));
        solutions.push(solve_greedy_nearest_y_first(positions.clone()));

        let mut min_score = usize::MAX;
        let mut new_best_solution: Option<Vec<u8>> = None;
        for solution in solutions.iter() {
            match solution {
                Some(moves) => {
                    if moves.len() < min_score {
                        new_best_solution = Some(moves.clone());
                    }
                },
                None => {
                }
            }
        }

        match new_best_solution {
            Some(new_best) => {
                let mut need_save = false;
                match best_solution {
                    Some(old_best) => {
                        if new_best.len() < old_best.len() {
                            println!("New best score {}! (old best: {})", new_best.len(), old_best.len());
                            need_save = true;
                        }
                    },
                    None => {
                        println!("New best score {}!", new_best.len());
                        need_save = true;
                    }
                }
                if need_save {
                    write_to_file(&solution_filepath, &new_best);
                }
            },
            None => {
                println!("Solution not found");
            }
        }
    }
}
