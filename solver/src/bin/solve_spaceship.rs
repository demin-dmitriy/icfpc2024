use std::env;
use std::num::NonZeroUsize;
use solver::{self, solve_spaceship_greedy::*};
use threadpool::ThreadPool;

fn run_solvers(
    problem_filepath: String,
    solution_filepath: String
) {
    let positions = read_positions(&problem_filepath);
    let best_solution = read_from_file(&solution_filepath);
    
    let mut solvers: Vec<(&str, fn(Vec<Pos>) -> Option<Vec<Step>>)> = Vec::new();
    //solvers.push(("Greedy max speed", solve_greedy));
    //solvers.push(("Greedy move towards", solve_greedy_towards));
    //solvers.push(("Greedy try min, fallback max speed", solve_greedy_try_min));
    //solvers.push(("Greedy try min, fallback towards", solve_greedy_try_min_and_towards));
    //solvers.push(("Greedy use quadtree, try min, fallback towards", solve_greedy_quadtree_try_min_and_towards));
    //solvers.push(("Greedy use quadtree, exact min, fallback towards", solve_greedy_quadtree_exact_min_and_towards));
    //solvers.push(("Greedy min step, exact min", solve_greedy_min_step_exact_move));
    //solvers.push(("Greedy min step iterative, exact min", solve_greedy_min_step_iterative_exact_move));
    solvers.push(("Greedy min step random, exact min", solve_greedy_min_step_random_exact_move));

    let mut min_score = usize::MAX;
    let mut new_best_solution: Option<Vec<Step>> = None;
    for (solver_name, solver) in solvers.iter() {
        println!("Task '{}', run solver '{}'", problem_filepath, solver_name);
        let solution = solver(positions.clone());
        match solution {
            Some(moves) => {
                println!("Task '{}', score: {} (solver '{}')", problem_filepath, moves.len(), solver_name);
                if moves.len() < min_score {
                    new_best_solution = Some(moves.clone());
                    min_score = moves.len();
                }
            },
            None => {
                println!("Task '{}', solution not found (solver '{}')", problem_filepath, solver_name);
            }
        }
    }

    match new_best_solution {
        Some(new_best) => {
            let mut need_save = false;
            match best_solution {
                Some(old_best) => {
                    if new_best.len() < old_best.len() {
                        println!("Task '{}', new best score: {}! (old best: {})", problem_filepath, new_best.len(), old_best.len());
                        need_save = true;
                    }
                },
                None => {
                    println!("Task '{}', new best score {}!", problem_filepath, new_best.len());
                    need_save = true;
                }
            }
            if need_save {
                write_to_file(&solution_filepath, &new_best);
            }
        },
        None => {
            println!("Task '{}', solution not found", problem_filepath);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);
    let problems_folder = &args[1];
    let solutions_folder = &args[2];

    const DEFAULT_WORKERS_COUNT: NonZeroUsize = nonzero_lit::usize!(2);
    let workers_by_default = std::thread::available_parallelism().unwrap_or(DEFAULT_WORKERS_COUNT);

    let pool = ThreadPool::new(workers_by_default.get());

    for i in 23..26 {
        let problem_filepath = format!("{}/{}", problems_folder, i);
        let solution_filepath = format!("{}/{}", solutions_folder, i);
        pool.execute(
            move || 
            {
                run_solvers(problem_filepath, solution_filepath);
            }
        );
    }
    pool.join();
}
