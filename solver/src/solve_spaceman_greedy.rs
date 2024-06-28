 
use std::fs::read_to_string;

#[derive(PartialEq)]
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

fn add(pos: &Pos, speed: &Pos) -> Pos {
    Pos {
        x: pos.x + speed.x,
        y: pos.y + speed.y
    }
}

struct Move {
    diff: Pos,
    code: u8
}

pub fn solve_greedy(mut positions: Vec<Pos>) {
    let mut moves = Vec::new();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut current_speed = Pos{x: 0, y: 0};

    let all_moves = vec![
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

    while !positions.is_empty() {
        let nearest = get_nearest_and_remove(&current_pos, &mut positions);

        for m in all_moves.iter() {
            let new_speed = add(&current_speed, &m.diff);
            if add(&current_pos, &new_speed) == nearest {
                current_speed = new_speed;
                current_pos = add(&current_pos, &current_speed);
                moves.push(m.code);
                break;
            }
        }

        if current_pos != nearest {
            println!("Failure!");
            return;
        }
    }

    for m in moves {
        print!("{}", m);
    }
    println!("");
}