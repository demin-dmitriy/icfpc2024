 
use std::fs::read_to_string;
use std::io::Write;

use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};
use rand::Rng;

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

#[derive(Clone, Copy)]
struct Move {
    diff: Pos,
    code: u8
}

const MOVE_LIMIT: usize = 10000000;

const NOOP_MOVE: Move = Move{diff: Pos{x: 0, y: 0}, code: 5};
const ALL_MOVES: [Move; 9] = [
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

#[derive(Clone, Copy)]
pub struct Step {
    m: Move,
    result_pos: Pos,
    result_speed: Pos
}

fn towards_to(
    current_pos: &Pos,
    current_speed: &Pos,
    target: &Pos,
) -> Step {
    let mut dist = f64::MAX;
    let mut best_move = NOOP_MOVE;
    for m in ALL_MOVES.iter() {
        let new_speed = add(&current_speed, &m.diff);

        let new_pos = add(&current_pos, &new_speed);
        let new_dist = distance(&new_pos, &target);
        if new_dist < dist {
            best_move = m.clone();
            dist = new_dist;
        }
    }

    let new_speed = add(&current_speed, &best_move.diff);
    Step{
        m: best_move,
        result_pos: add(&current_pos, &new_speed),
        result_speed: new_speed
    }
}

fn move_to(
    mut current_pos: Pos,
    mut current_speed: Pos,
    target: &Pos,
    all_moves_count: usize
) -> Option<Vec<Step>> {

    let mut dist = f64::MAX;
    let mut moves_to_target = Vec::new();
    while dist > 0.0 {
        if all_moves_count + moves_to_target.len() > MOVE_LIMIT {
            return None;
        }

        let new_move = towards_to(
            &current_pos, 
            &current_speed, 
            target, 
        );
        current_pos = new_move.result_pos;
        current_speed = new_move.result_speed;
        moves_to_target.push(new_move);

        dist = distance(&current_pos, &target);
    }

    Some(moves_to_target)
}

pub fn solve_greedy_with_init(
    mut positions: Vec<Pos>,
    init_pos: Pos,
    init_speed: Pos,
    init_moves: Vec<Step>
) -> Option<Vec<Step>> {
    let mut all_moves = init_moves;
    let mut current_pos = init_pos;
    let mut current_speed = init_speed;

    while !positions.is_empty() {
        let nearest = get_nearest_and_remove(&current_pos, &mut positions);

        let new_moves = move_to(
            current_pos, 
            current_speed, 
            &nearest, 
            all_moves.len()
        );
        match new_moves {
            Some(ms) => {
                all_moves.extend_from_slice(&ms);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
            },
            None => return None
        }
    }

    Some(all_moves)
}

pub fn solve_greedy_towards_with_init(
    mut positions: Vec<Pos>,
    init_pos: Pos,
    init_speed: Pos,
    init_moves: Vec<Step>
) -> Option<Vec<Step>> {
    let mut all_moves = init_moves;
    let mut current_pos = init_pos;
    let mut current_speed = init_speed;

    while !positions.is_empty() {
        let nearest = get_nearest_and_remove(&current_pos, &mut positions);

        let new_move = towards_to(
            &mut current_pos, 
            &mut current_speed, 
            &nearest
        );
        current_pos = new_move.result_pos;
        current_speed = new_move.result_speed;
        all_moves.push(new_move);

        if current_pos != nearest {
            positions.push(nearest)
        }

        if all_moves.len() > MOVE_LIMIT {
            return None;
        }
    }

    Some(all_moves)
}

pub fn solve_greedy(positions: Vec<Pos>) -> Option<Vec<Step>> {
    solve_greedy_with_init(
        positions, 
        Pos{x: 0, y: 0}, 
        Pos{x: 0, y: 0},
        Vec::new()
    )
}

pub fn solve_greedy_towards(positions: Vec<Pos>) -> Option<Vec<Step>> {
    solve_greedy_towards_with_init(
        positions, 
        Pos{x: 0, y: 0}, 
        Pos{x: 0, y: 0},
        Vec::new()
    )
}

fn move_min_to(
    current_pos: &Pos,
    current_speed: &Pos,
    target: &Pos,
    current_best: &mut Option<Vec<Step>>,
    current_moves: &Vec<Step>,
    all_moves_count: usize
) {
    if current_moves.len() > 3 {
        return;
    }
    if all_moves_count + current_moves.len() > MOVE_LIMIT {
        return;
    }

    match current_best {
        Some(best) => {
            if current_moves.len() >= best.len() {
                return;
            }
        },
        None => { }
    }

    let noop_speed = Pos{x: 0, y: 0};

    for m in ALL_MOVES {
        let new_speed = add(&current_speed, &m.diff);
        if new_speed == noop_speed {
            continue;
        }

        let new_pos = add(&current_pos, &new_speed);
        let step = Step{
            m,
            result_pos: new_pos,
            result_speed:  new_speed
        };

        if new_pos == *target {
            match current_best {
                Some(best) => {
                    if current_moves.len() + 1 < best.len() {
                        let mut moves = current_moves.clone();
                        moves.push(step);
                        *current_best = Some(moves);
                    }
                },
                None => {
                    let mut moves = current_moves.clone();
                    moves.push(step);
                    *current_best = Some(moves);
                }
            }
            return;
        }

        let mut moves = current_moves.clone();
        moves.push(step);
        move_min_to(
            &new_pos, 
            &new_speed, 
            target, 
            current_best,
            &moves,
            all_moves_count
        );
    }
}

pub fn solve_greedy_try_min(mut positions: Vec<Pos>) -> Option<Vec<Step>>  {
    let mut all_moves = Vec::new();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut current_speed = Pos{x: 0, y: 0};

    while !positions.is_empty() {
        let nearest = get_nearest_and_remove(&current_pos, &mut positions);

        let mut current_best = move_to(
            current_pos, 
            current_speed, 
            &nearest, 
            all_moves.len()
        );

        move_min_to(
            &current_pos, 
            &current_speed, 
            &nearest, 
            &mut current_best, 
            &Vec::new(), 
            all_moves.len()
        );

        match current_best {
            Some(ms) => {
                all_moves.extend_from_slice(&ms);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
            },
            None => return None
        }
        if all_moves.len() > MOVE_LIMIT {
            return None;
        }
    }

    Some(all_moves)
}

pub fn solve_greedy_try_min_and_towards(mut positions: Vec<Pos>) -> Option<Vec<Step>>  {
    let mut all_moves = Vec::new();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut current_speed = Pos{x: 0, y: 0};

    while !positions.is_empty() {
        let nearest = get_nearest_and_remove(&current_pos, &mut positions);

        let mut current_best = None;

        move_min_to(
            &current_pos, 
            &current_speed, 
            &nearest, 
            &mut current_best, 
            &Vec::new(), 
            all_moves.len()
        );

        match current_best {
            Some(ms) => {
                all_moves.extend_from_slice(&ms);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
            },
            None => {
                let new_move = towards_to(
                    &current_pos, 
                    &current_speed, 
                    &nearest
                );
                all_moves.push(new_move);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
                if current_pos != nearest {
                    positions.push(nearest)
                }
            }
        }
        if all_moves.len() > MOVE_LIMIT {
            return None;
        }
    }

    Some(all_moves)
}

pub fn solve_greedy_quadtree_try_min_and_towards(positions: Vec<Pos>) -> Option<Vec<Step>>  {
    let mut all_moves = Vec::new();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut current_speed = Pos{x: 0, y: 0};

    let mut padding = Pos{x: 0, y: 0};
    for pos in &positions {
        padding.x = padding.x.min(pos.x);
        padding.y = padding.y.min(pos.y);
    }
    padding.x = -padding.x;
    padding.y = -padding.y;

    let mut qt = Quadtree::<i64, Pos>::new(30);
    for pos in &positions {
        qt.insert_pt(Point{x: padding.x + pos.x, y: padding.y + pos.y}, pos.clone());
    }

    while !qt.is_empty() {
        let mut search_region_size = 10;
        let mut points_near = Vec::new();
        while points_near.is_empty() {            
            let region = AreaBuilder::default()
                .anchor(
                    Point{
                        x: (padding.x + current_pos.x - search_region_size / 2).max(0), 
                        y: (padding.y + current_pos.y - search_region_size / 2).max(0)
                    }
                )
                .dimensions((search_region_size, search_region_size))
                .build().unwrap();
            let query = qt.query(region);
            for v in query {
                points_near.push(v.value_ref().clone());
            }

            if points_near.is_empty() {
                search_region_size += 5;
            }
        }
        let nearest = get_nearest_and_remove(&current_pos, &mut points_near);
        {
            let region = AreaBuilder::default()
                .anchor(Point{x: padding.x + nearest.x, y: padding.y + nearest.y})
                .dimensions((1, 1))
                .build().unwrap();
            qt.delete(region);
        }

        let mut current_best = None;

        move_min_to(
            &current_pos, 
            &current_speed, 
            &nearest, 
            &mut current_best, 
            &Vec::new(), 
            all_moves.len()
        );

        match current_best {
            Some(ms) => {
                all_moves.extend_from_slice(&ms);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
            },
            None => {
                let new_move = towards_to(
                    &current_pos, 
                    &current_speed, 
                    &nearest
                );
                all_moves.push(new_move);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
                if current_pos != nearest {
                    qt.insert_pt(Point{x: padding.x + nearest.x, y: padding.y + nearest.y}, nearest);
                }
            }
        }
        if all_moves.len() > MOVE_LIMIT {
            return None;
        }
    }

    Some(all_moves)
}

fn is_reachable_1d(
    current_pos: i64,
    current_speed: i64,
    target: i64,
    steps: i64
) -> bool {
    let center = current_pos + current_speed * steps;
    let radius = (steps * (steps + 1)) / 2;

    (target >= center - radius) && (target <= center + radius)
}

fn is_reachable(
    current_pos: &Pos,
    current_speed: &Pos,
    target: &Pos,
    steps: i64
) -> bool {
    
    is_reachable_1d(current_pos.x, current_speed.x, target.x, steps)
    && is_reachable_1d(current_pos.y, current_speed.y, target.y, steps)
}

// change speed at the beginning, that move with stable speed
fn move_using_n_steps_1d(
    mut current_pos: i64,
    mut current_speed: i64,
    target: i64,
    steps: i64
) -> Vec<i64> {
    let mut moves = Vec::new();

    for i in 0..steps {
        let future_pos = current_pos + current_speed * (steps - i);
        let m;
        if future_pos == target {
            m = 0;
        }
        else if future_pos > target {
            m = -1;
        }
        else {
            m = 1;
        }
        current_speed += m;
        current_pos += current_speed;
        moves.push(m);
    }

    moves
}

fn _move_using_n_steps_1d_inertion_first(
    mut current_pos: i64,
    mut current_speed: i64,
    target: i64,
    steps: i64
) -> Vec<i64> {
    let mut moves = Vec::new();

    for i in 0..steps {
        let m;
        if is_reachable_1d(current_pos + current_speed, current_speed, target, steps - i - 1) {
            m = 0;
        }
        else {
            let future_pos = current_pos + current_speed * (steps - i);
            if future_pos == target {
                m = 0;
            }
            else if future_pos > target {
                m = -1;
            }
            else {
                m = 1;
            }
        }
        current_speed += m;
        current_pos += current_speed;
        moves.push(m);
    }

    moves
}

fn to_2d_move(
    x: i64,
    y: i64
) -> Move {
    let pos = Pos{x, y};
    match x {
        -1 => {
            match y {
                -1 => Move{diff: pos, code: 1},
                0 => Move{diff: pos, code: 4},
                1 => Move{diff: pos, code: 7},
                _ => unreachable!()
            }
        },
        0 => {
            match y {
                -1 => Move{diff: pos, code: 2},
                0 => Move{diff: pos, code: 5},
                1 => Move{diff: pos, code: 8},
                _ => unreachable!()
            }
        },
        1 => {
            match y {
                -1 => Move{diff: pos, code: 3},
                0 => Move{diff: pos, code: 6},
                1 => Move{diff: pos, code: 9},
                _ => unreachable!()
            }
        },
        _ => unreachable!()
    }
}

fn move_using_n_steps(
    mut current_pos: Pos,
    mut current_speed: Pos,
    target: &Pos,
    steps: i64
) -> Vec<Step> {

    let moves_x = move_using_n_steps_1d(
        current_pos.x, 
        current_speed.x, 
        target.x, 
        steps
    );
    let moves_y = move_using_n_steps_1d(
        current_pos.y, 
        current_speed.y, 
        target.y, 
        steps
    );

    let mut moves = Vec::new();
    for i in 0..moves_x.len() {
        let x_move = moves_x[i];
        let y_move = moves_y[i];
        let m = to_2d_move(x_move, y_move);
        current_speed = add(&current_speed, &m.diff);
        current_pos = add(&current_pos, &current_speed);
        let step = Step{
            m,
            result_pos: current_pos,
            result_speed: current_speed
        };
        moves.push(step);
    }
    assert!(moves.last().unwrap().result_pos == *target);
    moves
}

fn move_to_exact(
    current_pos: &Pos,
    current_speed: &Pos,
    target: &Pos,
    all_moves_count: usize
) -> Option<Vec<Step>> {
    for steps in 1..1000 {
        if all_moves_count + steps > MOVE_LIMIT {
            return None;
        }
        
        let target_reachable = is_reachable(&current_pos, &current_speed, target, steps as i64);
        if !target_reachable {
            continue;
        }

        return Some(
            move_using_n_steps(current_pos.clone(), current_speed.clone(), target, steps as i64)
        );
    }
    None
}

pub fn solve_greedy_quadtree_exact_min_and_towards(positions: Vec<Pos>) -> Option<Vec<Step>>  {
    let mut all_moves = Vec::new();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut current_speed = Pos{x: 0, y: 0};

    let mut padding = Pos{x: 0, y: 0};
    for pos in &positions {
        padding.x = padding.x.min(pos.x);
        padding.y = padding.y.min(pos.y);
    }
    padding.x = -padding.x;
    padding.y = -padding.y;

    let mut qt = Quadtree::<i64, Pos>::new(30);
    for pos in &positions {
        qt.insert_pt(Point{x: padding.x + pos.x, y: padding.y + pos.y}, pos.clone());
    }

    while !qt.is_empty() {
        let mut search_region_size = 10;
        let mut points_near = Vec::new();
        while points_near.is_empty() {            
            let region = AreaBuilder::default()
                .anchor(
                    Point{
                        x: (padding.x + current_pos.x - search_region_size / 2).max(0), 
                        y: (padding.y + current_pos.y - search_region_size / 2).max(0)
                    }
                )
                .dimensions((search_region_size, search_region_size))
                .build().unwrap();
            let query = qt.query(region);
            for v in query {
                points_near.push(v.value_ref().clone());
            }

            if points_near.is_empty() {
                search_region_size += 5;
            }
        }
        let nearest = get_nearest_and_remove(&current_pos, &mut points_near);
        {
            let region = AreaBuilder::default()
                .anchor(Point{x: padding.x + nearest.x, y: padding.y + nearest.y})
                .dimensions((1, 1))
                .build().unwrap();
            qt.delete(region);
        }

        let new_moves =  move_to_exact(
            &current_pos, 
            &current_speed, 
            &nearest, 
            all_moves.len()
        );
        match new_moves {
            Some(ms) => {
                all_moves.extend_from_slice(&ms);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
            },
            None => {
                let new_move = towards_to(
                    &current_pos, 
                    &current_speed, 
                    &nearest
                );
                all_moves.push(new_move);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
                if current_pos != nearest {
                    qt.insert_pt(Point{x: padding.x + nearest.x, y: padding.y + nearest.y}, nearest);
                }
            }
        }
        if all_moves.len() > MOVE_LIMIT {
            return None;
        }
    }

    Some(all_moves)
}

fn step_distance(
    current_pos: &Pos,
    current_speed: &Pos,
    target: &Pos,
) -> i64 {
    let mut steps = 1;
    loop {
        let target_reachable = is_reachable(&current_pos, &current_speed, target, steps);
        if target_reachable {
            return steps;
        }

        steps += 1;
    }
}

fn get_min_step_nearest_and_remove(
    current_pos: &Pos, 
    current_speed: &Pos,
    positions: &mut Vec<Pos>
) -> Pos {
    assert!(positions.len() > 0);

    let mut min_index = 0;
    let mut min_dist = step_distance(current_pos, current_speed, &positions[min_index]);
    for i in 1..positions.len() {
        let dist = step_distance(current_pos, current_speed, &positions[i]);
        if dist < min_dist {
            min_dist = dist;
            min_index = i;
        }
    }
    let result = positions.remove(min_index);
    result
}

pub fn solve_greedy_min_step_exact_move(mut positions: Vec<Pos>) -> Option<Vec<Step>>  {
    let mut all_moves = Vec::new();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut current_speed = Pos{x: 0, y: 0};

    while !positions.is_empty() {
        let nearest = get_min_step_nearest_and_remove(&current_pos, &current_speed, &mut positions);

        let new_moves =  move_to_exact(
            &current_pos, 
            &current_speed, 
            &nearest, 
            all_moves.len()
        );

        match new_moves {
            Some(ms) => {
                all_moves.extend_from_slice(&ms);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
            },
            None => {
                let new_move = towards_to(
                    &current_pos, 
                    &current_speed, 
                    &nearest
                );
                all_moves.push(new_move);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
                if current_pos != nearest {
                    positions.push(nearest)
                }
            }
        }
        if all_moves.len() > MOVE_LIMIT {
            return None;
        }
    }

    Some(all_moves)
}

pub fn solve_greedy_min_step_iterative_exact_move(mut positions: Vec<Pos>) -> Option<Vec<Step>>  {
    let mut all_moves = Vec::new();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut current_speed = Pos{x: 0, y: 0};

    while !positions.is_empty() {
        let nearest = get_min_step_nearest_and_remove(&current_pos, &current_speed, &mut positions);

        let new_moves =  move_to_exact(
            &current_pos, 
            &current_speed, 
            &nearest, 
            all_moves.len()
        );

        match new_moves {
            Some(ms) => {
                all_moves.push(*ms.first().unwrap());
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
                if current_pos != nearest {
                    positions.push(nearest)
                }
            },
            None => {
                let new_move = towards_to(
                    &current_pos, 
                    &current_speed, 
                    &nearest
                );
                all_moves.push(new_move);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
                if current_pos != nearest {
                    positions.push(nearest)
                }
            }
        }
        if all_moves.len() > MOVE_LIMIT {
            return None;
        }
    }

    Some(all_moves)
}

fn get_random_min_step_nearest_and_remove(
    current_pos: &Pos, 
    current_speed: &Pos,
    positions: &mut Vec<Pos>
) -> Pos {
    assert!(positions.len() > 0);

    let mut min_dist = step_distance(current_pos, current_speed, &positions[0]);

    let mut min_subset = Vec::new();
    min_subset.push(0);
    for i in 1..positions.len() {
        let dist = step_distance(current_pos, current_speed, &positions[i]);
        if dist < min_dist {
            min_dist = dist;
            min_subset.clear();
            min_subset.push(i);
        }
        else if dist == min_dist {
            min_subset.push(i);
        }
    }

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..min_subset.len());

    let result = positions.remove(min_subset[index]);
    result
}

pub fn solve_greedy_min_step_random_exact_move(mut positions: Vec<Pos>) -> Option<Vec<Step>>  {
    let mut all_moves = Vec::new();
    let mut current_pos = Pos{x: 0, y: 0};
    let mut current_speed = Pos{x: 0, y: 0};

    while !positions.is_empty() {
        let nearest = get_random_min_step_nearest_and_remove(&current_pos, &current_speed, &mut positions);

        let new_moves =  move_to_exact(
            &current_pos, 
            &current_speed, 
            &nearest, 
            all_moves.len()
        );

        match new_moves {
            Some(ms) => {
                all_moves.extend_from_slice(&ms);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
            },
            None => {
                let new_move = towards_to(
                    &current_pos, 
                    &current_speed, 
                    &nearest
                );
                all_moves.push(new_move);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
                if current_pos != nearest {
                    positions.push(nearest)
                }
            }
        }
        if all_moves.len() > MOVE_LIMIT {
            return None;
        }
    }

    Some(all_moves)
}

pub fn solve_tsp_exact_move(mut positions: Vec<Pos>) -> Option<Vec<Step>>  {
    let initial_pos = Pos{x: 0, y: 0};
    let mut all_moves = Vec::new();
    let mut current_pos = initial_pos;
    let mut current_speed = Pos{x: 0, y: 0};

    let mut tsp_points = Vec::new();
    for pos in &positions {
        tsp_points.push((pos.x as f64, pos.y as f64));
    }
    let tour = travelling_salesman::simulated_annealing::solve(
        &tsp_points,
        time::Duration::seconds(600),
    );
    println!("Tour distance: {}", tour.distance);

    let mut step = 0;
    for index in tour.route {
        step += 1;

        let next = &positions[index];

        let new_moves =  move_to_exact(
            &current_pos, 
            &current_speed, 
            &next, 
            all_moves.len()
        );

        match new_moves {
            Some(ms) => {
                all_moves.extend_from_slice(&ms);
                current_pos = all_moves.last().unwrap().result_pos;
                current_speed = all_moves.last().unwrap().result_speed;
            },
            None => {
                return None;
            }
        }
        if all_moves.len() > MOVE_LIMIT {
            return None;
        }
        println!("Point {}/{}, total moves {}", step, positions.len(), all_moves.len());
    }

    Some(all_moves)
}

pub fn print_to_console(moves: &[Step]) {
    for step in moves {
        print!("{}", step.m.code);
    }
    println!("");
}

pub fn read_from_file(filepath: &str) -> Option<String> {
    match std::fs::read_to_string(filepath) {
        Ok(result) => Some(result),
        Err(_) => None
    }
}

pub fn write_to_file(filepath: &str, moves: &[Step]) {
    let output = std::fs::File::create(filepath).unwrap();
    let mut output = std::io::BufWriter::new(output);
    for step in moves {
        write!(output, "{}", step.m.code).unwrap();
    }
}