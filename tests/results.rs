
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

#[cfg(test)]
mod day2 {
    use aoc2022::day2;
    use aoc2022::day2::{string_to_rps, string_to_rps2};

    #[test]
    fn part1() {
        let input = day2::path_to_rps("data/day2.txt" , string_to_rps);

        assert_eq!(10941, day2::part1(input));
    }

    #[test]
    fn part2() {
        let input = day2::path_to_rps("data/day2.txt", string_to_rps2);

        assert_eq!(13071, day2::part2(input));
    }

}
