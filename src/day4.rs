
pub fn part1(input: Vec<String>) -> u32 {
    input.iter()
        .map(|line| parse_regions(line))
        .map(|(x1, x2, y1, y2)| is_region_fully_contained(x1, x2, y1, y2))
        .map(|b| b as u32)
        .sum()
}

pub fn part2(input: Vec<String>) -> u32 {
    input.iter()
        .map(|line| parse_regions(line))
        .map(|(x1, x2, y1, y2)| is_region_partially_contained(x1, x2, y1, y2))
        .map(|b| b as u32)
        .sum()
}

fn parse_regions(line: &String) -> (u32, u32, u32, u32) {
    let mut split = line.split(&['-', ','])
    .map(|x| x.parse::<u32>().unwrap());

    (split.next().unwrap(), split.next().unwrap(), split.next().unwrap(), split.next().unwrap())
}

fn is_region_partially_contained(x1: u32, x2: u32, y1: u32, y2: u32) -> bool {
    (x1 >= y1 && x1 <= y2) || (x2 >= y1 && x2 <= y2) ||
        (y1 >= x1 && y1 <= x2) || (y2 >= x1 && y2 <= x2)
}

fn is_region_fully_contained(x1: u32, x2: u32, y1: u32, y2: u32) -> bool {
    let first_overlaps_second = (x1 >= y1 && x1 <= y2) && (x2 >= y1 && x2 <= y2);
    let second_overlaps_first = (y1 >= x1 && y1 <= x2) && (y2 >= x1 && y2 <= x2);

    first_overlaps_second || second_overlaps_first
}


#[cfg(test)]
mod tests {
    use crate::day4::{part1, part2};
    use crate::parsing::parse_to_strings;

    static EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn sample1() {
        let v= EXAMPLE_INPUT.lines()
            .map(|l| l.to_string())
            .collect();
        assert_eq!(2, part1(v));
    }

    #[test]
    fn sample2() {
        let v= EXAMPLE_INPUT.lines()
            .map(|l| l.to_string())
            .collect();
        assert_eq!(4, part2(v));
    }

}

