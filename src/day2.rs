use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::os::unix::raw::ino_t;
use std::str::FromStr;
use crate::day2::MatchResult::{DRAW, LOSS, WIN};

#[derive(Debug)]
pub enum RPS {
    ROCK, PAPER, SCISSORS
}

impl FromStr for RPS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPS::ROCK)
            , "B" => Ok(RPS::PAPER)
            , "C" => Ok(RPS::SCISSORS)
            , "X" => Ok(RPS::ROCK)
            , "Y" => Ok(RPS::PAPER)
            , "Z" => Ok(RPS::SCISSORS)
            , _ => Err(())
        }
    }
}

#[derive(Debug)]
pub struct Round {
    player: RPS
    , opponent: RPS
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");

        Ok(Round {
            opponent: RPS::from_str(split.next().unwrap()).unwrap()
            , player: RPS::from_str(split.next().unwrap()).unwrap()
        })
    }
}


#[derive(Debug)]
pub enum MatchResult {
    WIN, LOSS, DRAW
}

impl FromStr for MatchResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(LOSS)
            , "Y" => Ok(DRAW)
            , "Z" => Ok(WIN)
            , _ => Err(())
        }
    }
}

#[derive(Debug)]
pub struct Round2 {
    player: MatchResult
    , opponent: RPS
}

impl FromStr for Round2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");

        Ok(Round2 {
            opponent: RPS::from_str(split.next().unwrap()).unwrap()
            , player: MatchResult::from_str(split.next().unwrap()).unwrap()
        })
    }
}


pub fn part1(input: Vec<Round>) -> u32 {
    input.iter()
        .map(|round| match round {
            Round {player: RPS::ROCK, opponent: RPS::ROCK} => 1 + 3
            , Round {player: RPS::ROCK, opponent: RPS::PAPER} => 1 + 0
            , Round {player: RPS::ROCK, opponent: RPS::SCISSORS} => 1 + 6

            , Round {player: RPS::PAPER, opponent: RPS::ROCK} => 2 + 6
            , Round {player: RPS::PAPER, opponent: RPS::PAPER} => 2 + 3
            , Round {player: RPS::PAPER, opponent: RPS::SCISSORS} => 2 + 0

            , Round {player: RPS::SCISSORS, opponent: RPS::ROCK} => 3 + 0
            , Round {player: RPS::SCISSORS, opponent: RPS::PAPER} => 3 + 6
            , Round {player: RPS::SCISSORS, opponent: RPS::SCISSORS} => 3 + 3
            , _ => 0 })
        .sum::<u32>()
}

pub fn part2(input: Vec<Round2>) -> u32 {
    input.iter()
        .map(|round| match round {
            Round2 { player: LOSS, opponent: RPS::ROCK } => 0 + 3
            , Round2 { player: LOSS, opponent: RPS::PAPER } => 0 + 1
            , Round2 { player: LOSS, opponent: RPS::SCISSORS } => 0 + 2

            , Round2 { player: DRAW, opponent: RPS::ROCK } => 3 + 1
            , Round2 { player: DRAW, opponent: RPS::PAPER } => 3 + 2
            , Round2 { player: DRAW, opponent: RPS::SCISSORS } => 3 + 3

            , Round2 { player: WIN, opponent: RPS::ROCK } => 6 + 2
            , Round2 { player: WIN, opponent: RPS::PAPER } => 6 + 3
            , Round2 { player: WIN, opponent: RPS::SCISSORS } => 6 + 1
            , _ => 0
        })
        .sum()
}


pub fn path_to_rps<R>(path: &str, parsing_fn: fn(String) -> Vec<R>) -> Vec<R> {

    let file = File::open(path)
        .expect("File not found");
    let mut reader = BufReader::new(file);

    let mut s = String::new();
    reader.read_to_string(&mut s);

    parsing_fn(s)
}

pub fn string_to_rps(s: String) -> Vec<Round> {
    s.lines()
        .map(|line| Round::from_str(line).unwrap())
        .collect()
}

pub fn string_to_rps2(s: String) -> Vec<Round2> {
    s.lines()
        .map(|line| Round2::from_str(line).unwrap())
        .collect()
}


#[cfg(test)]
mod tests {
    use crate::day2::{path_to_rps, part1, string_to_rps, part2, string_to_rps2};

    static EXAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn sample1() {
        let v = string_to_rps(EXAMPLE_INPUT.to_string());

        assert_eq!(15, part1(v));
    }

    #[test]
    fn sample2() {
        let v = string_to_rps2(EXAMPLE_INPUT.to_string());

        assert_eq!(12, part2(v));
    }

    #[test]
    fn test_parsing() {
        path_to_rps("data/day2.txt", string_to_rps);
    }

}
