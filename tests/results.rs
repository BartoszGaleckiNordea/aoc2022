
#[cfg(test)]
mod day1 {

    use aoc2022::day1;

    #[test]
    fn part1() {
        let input = day1::parse_to_vec("data/day1.txt");

        assert_eq!(69693, day1::part1(input));
    }

    #[test]
    fn part2() {
        let input = day1::parse_to_vec("data/day1.txt");

        assert_eq!(200945, day1::part2(input));
    }

}
