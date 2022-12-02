#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Goal {
    Win,
    Lose,
    Draw,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameResult {
    PlayerWon,
    OpponentWon,
    Draw,
}

use Shape::*;

fn player_shape(opp: Shape, goal: Goal) -> Shape {
    match goal {
        Goal::Win => match opp {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
        Goal::Lose => match opp {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        Goal::Draw => opp,
    }
}

fn game_result(opp: Shape, you: Shape) -> GameResult {
    if opp == you {
        return GameResult::Draw;
    } else {
        match opp {
            Rock => {
                if you == Paper {
                    GameResult::PlayerWon
                } else {
                    GameResult::OpponentWon
                }
            }
            Paper => {
                if you == Scissors {
                    GameResult::PlayerWon
                } else {
                    GameResult::OpponentWon
                }
            }
            Scissors => {
                if you == Rock {
                    GameResult::PlayerWon
                } else {
                    GameResult::OpponentWon
                }
            }
        }
    }
}

fn score(opp: Shape, you: Shape) -> u32 {
    (match game_result(opp, you) {
        GameResult::PlayerWon => 6,
        GameResult::OpponentWon => 0,
        GameResult::Draw => 3,
    }) + (match you {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    })
}

fn parse_line(line: &str) -> (Shape, Goal) {
    let opp = match line.chars().nth(0).unwrap() {
        'A' => Rock,
        'B' => Paper,
        'C' => Scissors,
        _ => panic!("invalid opponenet shape"),
    };

    let you = match line.chars().nth(2).unwrap() {
        'X' => Goal::Lose,
        'Y' => Goal::Draw,
        'Z' => Goal::Win,
        _ => panic!("invalid player shape"),
    };

    (opp, you)
}

fn main() {
    let res: u32 = std::io::stdin()
        .lines()
        .map(|line| {
            let (opp, goal) = parse_line(&line.unwrap());
            let you = player_shape(opp, goal);
            score(opp, you)
        })
        .sum();
    println!("{}", res);
}
