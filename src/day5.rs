use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;

struct Instruction {
    amount: u32
    , from: u32
    , to: u32
}

lazy_static! {
    static ref NUMBER_FINDER: Regex = Regex::new(r"\d").unwrap();
}

impl FromStr for Instruction {

    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut matches = NUMBER_FINDER.find_iter(s)
            .map(|number| number.as_str().parse::<u32>().unwrap());

        Ok(Instruction {
            amount: matches.next().unwrap()
            , from: matches.next().unwrap()
            , to: matches.next().unwrap()
        })
    }
}

pub fn part1(input: String) -> String {
    let mut split = input.split("\n\n");
    let x = split.next().unwrap();

    let x1: Vec<Instruction> = split.next()
        .unwrap()
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    // println!("{:?}", split);
    println!("{:?}", split.next().unwrap());
    println!("{:?}", split.next().unwrap());


    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::day5::part1;
    use crate::parsing::parse_to_string;

    const EXAMPLE_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn sample1() {


        assert_eq!("ABC", part1(EXAMPLE_INPUT.to_string()))
    }

}
