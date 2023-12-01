const PUZZLE_INPUT: &str = include_str!("input/2023_01.txt");

fn main() {
    println!("Part one: {}", part_one(PUZZLE_INPUT));
    println!("Part two: {}", part_two(PUZZLE_INPUT));
}

fn part_one(input: &str) -> u32 {
    input.lines().map(find_digits_only).sum()
}

fn find_digits_only(line: &str) -> u32 {
    let first = line
        .chars()
        .find(char::is_ascii_digit)
        .and_then(|c| c.to_digit(10))
        .unwrap();
    let last = line
        .chars()
        .rfind(char::is_ascii_digit)
        .and_then(|c| c.to_digit(10))
        .unwrap();

    first * 10 + last
}

fn part_two(input: &str) -> u32 {
    input.lines().map(find_digits_and_words).sum()
}

fn find_digits_and_words(line: &str) -> u32 {
    let first = (0..line.len())
        .map(|idx| &line[idx..])
        .find_map(number_from_prefix)
        .expect("No digit found in line.");
    let last = (0..line.len())
        .rev()
        .map(|idx| &line[idx..])
        .find_map(number_from_prefix)
        .unwrap();

    first * 10 + last
}

fn number_from_prefix(s: &str) -> Option<u32> {
    [
        "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven",
        "8", "eight", "9", "nine",
    ]
    .into_iter()
    .find(|&num| s.starts_with(num))
    .and_then(convert_number)
}

fn convert_number(s: &str) -> Option<u32> {
    match s {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

mod test {
    #[test]
    fn can_find_digits() {
        let test_input = "\
            1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet";
        assert_eq!(crate::part_one(test_input), 142);
    }

    #[test]
    fn can_find_written_numbers() {
        let test_input = "\
            two1nine\n\
            eightwoeighthree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen";
        assert_eq!(crate::part_two(test_input), 281);
    }
}
