 
use std::fs::read_to_string;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(PartialEq, Clone, Copy)]
pub struct Pos
{
    pub x: i64,
    pub y: i64
}

pub fn read_positions(filename: &str) -> Vec<Pos> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let numbers: Vec<&str> = line.split_ascii_whitespace().collect();
        assert!(numbers.len() == 2);
        let x: i64 = numbers[0].parse().unwrap();
        let y: i64 = numbers[1].parse().unwrap();
        result.push(Pos{x: x, y: y});
    }

    result
}

fn distance(a: &Pos, b: &Pos) -> f64 {
    (((b.x - a.x).pow(2) + (b.y - a.y).pow(2)) as f64).sqrt()
}

fn distance_x(a: &Pos, b: &Pos) -> i64 {
    (b.x - a.x).abs()
}

fn distance_y(a: &Pos, b: &Pos) -> i64 {
    (b.y - a.y).abs()
}

fn get_nearest_and_remove(current_pos: &Pos, positions: &mut Vec<Pos>) -> Pos {
    assert!(positions.len() > 0);

    let mut min_index = 0;
    let mut min_dist = distance(current_pos, &positions[min_index]);
    for i in 1..positions.len() {
        let dist = distance(current_pos, &positions[i]);
        if dist < min_dist {
            min_dist = dist;
            min_index = i;
        }
    }
    let result = positions.remove(min_index);
    result
}

fn get_nearest_x_and_remove(current_pos: &Pos, positions: &mut Vec<Pos>) -> Pos {
    assert!(positions.len() > 0);

    let mut min_index = 0;
    let mut min_dist = distance_x(current_pos, &positions[min_index]);
    for i in 1..positions.len() {
        let dist = distance_x(current_pos, &positions[i]);
        if dist < min_dist {
            min_dist = dist;
            min_index = i;
        }
    }
    let result = positions.remove(min_index);
    result
}

fn get_nearest_y_and_remove(current_pos: &Pos, positions: &mut Vec<Pos>) -> Pos {
    assert!(positions.len() > 0);

    let mut min_index = 0;
    let mut min_dist = distance_y(current_pos, &positions[min_index]);
    for i in 1..positions.len() {
        let dist = distance_y(current_pos, &positions[i]);
        if dist < min_dist {
            min_dist = dist;
            min_index = i;
        }
    }
    let result = positions.remove(min_index);
    result
}

fn add(pos: &Pos, speed: &Pos) -> Pos {
    Pos {
        x: pos.x + speed.x,
        y: pos.y + speed.y
    }
}

#[derive(Clone, Copy)]
struct Move {
    diff: Pos,
    code: u8
}

fn move_to(
    current_pos: &mut Pos,
    current_speed: &mut Pos,
    target: &Pos,
    mut moves: Vec<u8>
) -> Option<Vec<u8>> {
    let noop_move = Move{diff: Pos{x: 0, y: 0}, code: 5};
    let all_moves = [
        Move{diff: Pos{x: -1, y: -1}, code: 1},
        Move{diff: Pos{x: 0, y: -1}, code: 2},
        Move{diff: Pos{x: 1, y: -1}, code: 3},
        Move{diff: Pos{x: -1, y: 0}, code: 4},
        Move{diff: Pos{x: 0, y: 0}, code: 5},
        Move{diff: Pos{x: 1, y: 0}, code: 6},
        Move{diff: Pos{x: -1, y: 1}, code: 7},
        Move{diff: Pos{x: 0, y: 1}, code: 8},
        Move{diff: Pos{x: 1, y: 1}, code: 9},
    ];

    let mut dist = f64::MAX;
    while dist > 0.0 {
        dist = f64::MAX;
        let mut best_move = noop_move;
        for m in all_moves.iter() {
            let new_speed = add(&current_speed, &m.diff);
            let new_pos = add(&current_pos, &new_speed);
            let new_dist = distance(&new_pos, &target);
            if new_dist < dist {
                best_move = m.clone();
                dist = new_dist;
            }
        }

        let new_speed = add(&current_speed, &best_move.diff);
        *current_speed = new_speed;
        *current_pos = add(&current_pos, &current_speed);
        
        moves.push(best_move.code);

        dist = distance(&current_pos, &target);

        if moves.len() > 1000000 {
            return None;
        }
    }

    Some(moves)
}

pub fn solve_greedy_with_init(
    mut positions: Vec<Pos>,
    init_pos: Pos,
    init_speed: Pos,
    init_moves: Vec<u8>
) -> Option<Vec<u8>> {
    let mut moves = init_moves;
    let mut current_pos = init_pos;
    let mut current_speed = init_speed;

    while !positions.is_empty() {
        let nearest = get_nearest_and_remove(&current_pos, &mut positions);

        let new_moves = move_to(&mut current_pos, &mut current_speed, &nearest, moves);
        match new_moves {
            Some(ms) => moves = ms,
            None => return None
        }
        if moves.len() > 1000000 {
            return None;
        }
    }

    Some(moves)
}

pub fn solve_greedy(mut positions: Vec<Pos>) -> Option<Vec<u8>> {
    solve_greedy_with_init(
        positions, 
        Pos{x: 0, y: 0}, 
        Pos{x: 0, y: 0},
        Vec::new()
    )
}

pub fn solve_greedy_nearest_x_first(mut positions: Vec<Pos>) -> Option<Vec<u8>> {
    let mut moves = Vec::new();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut current_speed = Pos{x: 0, y: 0};

    let nearest_x = get_nearest_x_and_remove(&current_pos, &mut positions);
    let new_moves = move_to(&mut current_pos, &mut current_speed, &nearest_x, moves);
    match new_moves {
        Some(ms) => moves = ms,
        None => return None
    }

    solve_greedy_with_init(
        positions, 
        current_pos, 
        current_speed,        
        moves
    )
}

pub fn solve_greedy_nearest_y_first(mut positions: Vec<Pos>) -> Option<Vec<u8>> {
    let mut moves = Vec::new();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut current_speed = Pos{x: 0, y: 0};

    let nearest_y = get_nearest_y_and_remove(&current_pos, &mut positions);
    let new_moves = move_to(&mut current_pos, &mut current_speed, &nearest_y, moves);
    match new_moves {
        Some(ms) => moves = ms,
        None => return None
    }

    solve_greedy_with_init(
        positions, 
        current_pos, 
        current_speed,        
        moves
    )
}

pub fn print_to_console(moves: &[u8]) {
    for m in moves {
        print!("{}", m);
    }
    println!("");
}

pub fn read_from_file(filepath: &str) -> Option<String> {
    match std::fs::read_to_string(filepath) {
        Ok(result) => Some(result),
        Err(_) => None
    }
}

pub fn write_to_file(filepath: &str, moves: &[u8]) {
    let mut output = std::fs::File::create(filepath).unwrap();
    let mut output = std::io::BufWriter::new(output);
    for m in moves {
        write!(output, "{}", m);
    }
}