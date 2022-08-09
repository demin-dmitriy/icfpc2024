use serde::{Deserialize, Serialize};

// Specification: https://icfpcontest2021.github.io/spec-v4.1.pdf

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
struct Point(i32, i32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
struct Hole(Vec<Point>);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
struct Edge(i32, i32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Bonus {
    Globalist,
    BreakALeg,
    Wallhack,
    Supperflex,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
struct Figure {
    pub vertices: Vec<Point>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
struct ProblemBonus {
    pub position: Point,
    pub bonus: Bonus,
    pub problem: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
struct Problem {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bonuses: Vec<ProblemBonus>,
    pub hole: Hole,
    pub figure: Figure,
    pub epsilon: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
struct PoseBonus {
    pub bonus: Bonus,
    pub problem: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
struct Pose {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bonuses: Vec<PoseBonus>,
    pub vertices: Vec<Point>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge: Option<Edge>,
}

const ICFPC2021_0001_PROBLEM: &str = include_str!("icfpc2021-0001.problem");

fn main() {
    let problem: Problem = serde_json::from_str(ICFPC2021_0001_PROBLEM).unwrap();
    println!("--- Parsed: examples/icfpc2021-001.problem ---");
    println!("{:?}", problem);
    println!("---");
    println!("--- Serialized back to json ---");
    println!("{}", serde_json::to_string(&problem).unwrap());
    println!("---");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_icfpc2021_problem() {
        let problem: Problem = serde_json::from_str(ICFPC2021_0001_PROBLEM).unwrap();

        assert_eq!(problem.bonuses[0].bonus, Bonus::Globalist);
        assert_eq!(problem.hole.0[0], Point(45, 80));
    }
}
