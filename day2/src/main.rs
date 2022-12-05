use color_eyre::{eyre::eyre, eyre::Context};
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Play {
    type Err = color_eyre::Report;

    fn from_str(input: &str) -> Result<Play, Self::Err> {
        match input {
            "A" => Ok(Play::Rock),
            "X" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "Y" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            "Z" => Ok(Play::Scissors),
            _ => Err(eyre!("Invalid play")),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum RoundScore {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

fn convert_outcome_to_enum(play: &str, opponent: Play) -> color_eyre::Result<Play> {
    match (play, opponent) {
        ("X", Play::Rock) => Ok(Play::Scissors),
        ("X", Play::Paper) => Ok(Play::Rock),
        ("X", Play::Scissors) => Ok(Play::Paper),
        ("Y", _) => Ok(opponent),
        ("Z", Play::Rock) => Ok(Play::Paper),
        ("Z", Play::Paper) => Ok(Play::Scissors),
        ("Z", Play::Scissors) => Ok(Play::Rock),
        _ => Err(eyre!("Invalid play from outcome")),
    }
}

fn get_score(opponent: Play, you: Play) -> u64 {
    let score = match (opponent, you) {
        (Play::Rock, Play::Rock) => RoundScore::Draw,
        (Play::Rock, Play::Paper) => RoundScore::Win,
        (Play::Rock, Play::Scissors) => RoundScore::Loss,
        (Play::Paper, Play::Rock) => RoundScore::Loss,
        (Play::Paper, Play::Paper) => RoundScore::Draw,
        (Play::Paper, Play::Scissors) => RoundScore::Win,
        (Play::Scissors, Play::Rock) => RoundScore::Win,
        (Play::Scissors, Play::Paper) => RoundScore::Loss,
        (Play::Scissors, Play::Scissors) => RoundScore::Draw,
    };

    score as u64 + you as u64
}

fn get_score_for_play(filepath: &str) -> color_eyre::Result<u64> {
    let total: u64 = fs::read_to_string(filepath)
        .wrap_err(format!("reading {}", filepath))?
        .lines()
        .flat_map(|line| {
            let plays = line.split_whitespace().collect::<Vec<&str>>();

            match plays[..] {
                [a, b] => Play::from_str(a)
                    .and_then(|opp| Play::from_str(b).map(|you| get_score(opp, you))),
                _ => Err(eyre!("Invalid line input")),
            }
        })
        .sum();
    Ok(total)
}

fn get_score_for_play_2(filepath: &str) -> color_eyre::Result<u64> {
    let total: u64 = fs::read_to_string(filepath)
        .wrap_err(format!("reading {}", filepath))?
        .lines()
        .flat_map(|line| {
            let plays = line.split_whitespace().collect::<Vec<&str>>();

            match plays[..] {
                [a, b] => Play::from_str(a)
                    .and_then(|opp| convert_outcome_to_enum(b, opp).map(|you| get_score(opp, you))),
                _ => Err(eyre!("Invalid line input")),
            }
        })
        .sum();
    Ok(total)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let score = get_score_for_play("src/input.txt")?;
    println!("score: {}", score);

    let score = get_score_for_play_2("src/input.txt")?;
    println!("score 2: {}", score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_score, get_score_for_play, get_score_for_play_2, Play};

    #[test]
    fn it_get_score_for_paper_win() {
        let score = get_score(Play::Rock, Play::Paper);
        assert_eq!(score, 8);
    }

    #[test]
    fn it_get_score_for_rock_loss() {
        let score = get_score(Play::Paper, Play::Rock);
        assert_eq!(score, 1);
    }

    #[test]
    fn it_get_score_for_scissors_draw() {
        let score = get_score(Play::Scissors, Play::Scissors);
        assert_eq!(score, 6);
    }

    #[test]
    fn it_get_score_for_test_play() {
        let score = get_score_for_play("src/input-test.txt").unwrap();
        assert_eq!(score, 15);
    }

    #[test]
    fn it_get_score_for_test_play_2() {
        let score = get_score_for_play_2("src/input-test.txt").unwrap();
        assert_eq!(score, 12);
    }
}
