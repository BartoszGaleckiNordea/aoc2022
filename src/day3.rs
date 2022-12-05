use std::char;
use std::collections::HashSet;
use std::str::Chars;

pub fn part1(input: Vec<String>) -> u32 {

    input.iter()
        .map(|s| find_duplicates(s))
        .map(|c| map_to_priority(c))
        .sum()
}

pub fn part2(input: Vec<String>) -> u32 {
    input.iter()
        .as_slice()
        .chunks(3)
        .map(|chunk| find_duplicates2(chunk))
        .map(|c| map_to_priority(c))
        .sum()
}

fn find_duplicates2(slices: &[String]) -> Option<char> {
    let mut iter = slices.iter()
        .map(|slice| HashSet::from_iter(slice.chars()))
        .into_iter();
    let option = iter
        .next()
        .map(|set: HashSet<char>| iter.fold(set, |set1, set2| &set1 & &set2))
        .unwrap();
    option.iter()
        .map(|x| *x)
        .next()
}

fn map_to_priority(c: Option<char>) -> u32 {
    match c {
        None => 0,
        Some(character) => {
            if character.is_lowercase() {
                return character as u32 - 96
            }

            if character.is_uppercase() {
                return character as u32 - 64 + 26
            }

            return 0
        }
    }

}

fn find_duplicates(s: &String) -> Option<char> {
    let first_compartment: HashSet<char> = s[0..s.len() / 2]
        .chars()
        .collect();
    let second_compartment: HashSet<char> = s[s.len()/2 ..s.len()]
        .chars()
        .collect();

    let intersection: Vec<char> = first_compartment.intersection(&second_compartment)
        .map(|i| *i)
        .collect();

    intersection.iter()
        .map(|i| *i)
        .take(1)
        .next()
}


#[cfg(test)]
mod tests {
    use crate::day3::{find_duplicates, part1, part2};

    static EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    static EXAMPLE_INPUT2: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";

    static EXAMPLE_INPUT3: &str = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    static EXAMPLE_INPUT4: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";


    #[test]
    fn sample1() {
        let input: Vec<String> = EXAMPLE_INPUT.lines()
            .map(|line| line.to_string())
            .collect();

        let result = part1(input);

        assert_eq!(157, result)
    }

    #[test]
    fn sample2() {
        let input: Vec<String> = EXAMPLE_INPUT2.lines()
            .map(|line| line.to_string())
            .collect();


        let result = part2(input);

        assert_eq!(18, result)
    }

    #[test]
    fn sample3() {
        let input: Vec<String> = EXAMPLE_INPUT3.lines()
            .map(|line| line.to_string())
            .collect();


        let result = part2(input);

        assert_eq!(52, result)
    }

    #[test]
    fn sample4() {
        let input: Vec<String> = EXAMPLE_INPUT4.lines()
            .map(|line| line.to_string())
            .collect();


        let result = part2(input);

        assert_eq!(18 + 52, result)
    }


}
