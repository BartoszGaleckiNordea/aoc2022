use std::fs::File;
use std::io::{BufRead, BufReader, Read};


pub fn part1(input: Vec<Vec<u32>>) -> u32 {

    let result = input.iter()
        .map(|ints| ints.iter().sum())
        .max();

    result.unwrap()
}

pub fn part2(input: Vec<Vec<u32>>) -> u32 {
    let mut sums: Vec<u32> = input.iter()
        .map(|ints| ints.iter().sum())
        .collect();

    sums.sort_by(|a, b| b.cmp(a));

    sums.iter()
        .take(3)
        .sum()
}

pub fn parse_to_vec(path: &str) -> Vec<Vec<u32>> {
    let file = File::open(path)
        .expect("File not found");
    let mut reader = BufReader::new(file);

    let mut str_input = String::new();
    reader.read_to_string(&mut str_input);

    parse_string(str_input)
}

fn parse_string(s: String) -> Vec<Vec<u32>> {
    let input = s
        .split("\n\n")
        .map(|elf| elf
            .lines()
            .map(|item| item.parse::<u32>().unwrap())
            .collect())
        .collect();

    input
}

#[cfg(test)]
mod tests {
    use crate::day1::{parse_string, parse_to_vec, part1, part2};

    static EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn sample1() {
        let input = parse_string(EXAMPLE_INPUT.to_string());

        assert_eq!(24000, part1(input))
    }

    #[test]
    fn sample2() {
        let input = parse_string(EXAMPLE_INPUT.to_string());

        assert_eq!(45000, part2(input));
    }


    #[test]
    fn test_parsing() {
        let vec = parse_string(EXAMPLE_INPUT.to_string());

        println!("{:?}", vec);
        assert_eq!(5, vec.len())
    }


}
