use crate::utils;

const INPUT: &str = include_str!("input/2023_09.txt");

pub fn run() -> String {
    let (front, back) = predict_sum_ends(parse_input(INPUT));
    utils::both(back, front)
}

fn predict_sum_ends(histories: Vec<Vec<i32>>) -> (i32, i32) {
    let (fronts, backs): (Vec<i32>, Vec<i32>) = histories.iter().map(|v| predict_ends(v)).unzip();
    (fronts.into_iter().sum(), backs.into_iter().sum())
}

fn predict_ends(history: &[i32]) -> (i32, i32) {
    let triangle = difference_triangle(history);
    (predict_first(&triangle), predict_last(&triangle))
}

fn predict_first(triangle: &[Vec<i32>]) -> i32 {
    let first_nums: Vec<i32> = triangle.iter().map(|v| v[0]).rev().collect();
    let (initial, diffs) = first_nums.split_last().unwrap();
    let total_diff = diffs.iter().copied().fold(0, |left, right| right - left);
    initial - total_diff
}

fn predict_last(triangle: &[Vec<i32>]) -> i32 {
    triangle.iter().map(|v| v.last().unwrap()).copied().sum()
}

fn difference_triangle(xs: &[i32]) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = vec![];
    let mut current = xs.to_vec();
    while !current.iter().all(|&x| x == 0) {
        let next_diffs = differences(&current);
        out.push(current);
        current = next_diffs;
    }
    out.push(current);
    out
}

fn differences(xs: &[i32]) -> Vec<i32> {
    xs.windows(2).map(|window| window[1] - window[0]).collect()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::{difference_triangle, differences, parse_input, predict_ends, predict_sum_ends};

    const TEST_INPUT: &str = "\
        0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45";

    #[test]
    fn day9_test_parse_input() {
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];
        let parsed = parse_input(TEST_INPUT);
        assert_eq!(expected, parsed);
    }

    #[test]
    fn day9_differences() {
        let parsed = parse_input(TEST_INPUT);
        let xs = &parsed[0];
        let expected = vec![3; 5];
        assert_eq!(differences(xs), expected);
    }

    #[test]
    fn day9_difference_triangle() {
        let parsed = parse_input(TEST_INPUT);
        let xs = &parsed[0];
        let expected = vec![xs.to_vec(), vec![3; 5], vec![0; 4]];
        assert_eq!(difference_triangle(xs), expected);
    }

    #[test]
    fn day9_predict_last() {
        let parsed = parse_input(TEST_INPUT);
        let xs = &parsed[0];
        assert_eq!(predict_ends(xs).1, 18);
    }

    #[test]
    fn day9_predict_last_2() {
        let parsed = parse_input(TEST_INPUT);
        let xs = &parsed[1];
        assert_eq!(predict_ends(xs).1, 28);
    }

    #[test]
    fn day9_predict_last_3() {
        let parsed = parse_input(TEST_INPUT);
        let xs = &parsed[2];
        assert_eq!(predict_ends(xs).1, 68);
    }

    #[test]
    fn day9_predict_first_1() {
        let parsed = parse_input(TEST_INPUT);
        let xs = &parsed[0];
        assert_eq!(predict_ends(xs).0, -3);
    }

    #[test]
    fn day9_predict_first_2() {
        let parsed = parse_input(TEST_INPUT);
        let xs = &parsed[1];
        assert_eq!(predict_ends(xs).0, 0);
    }

    #[test]
    fn day9_predict_first_3() {
        let parsed = parse_input(TEST_INPUT);
        let xs = &parsed[2];
        assert_eq!(predict_ends(xs).0, 5);
    }

    #[test]
    fn day9_both_test_input() {
        let (front, back) = predict_sum_ends(parse_input(TEST_INPUT));
        assert_eq!(back, 114, "Part one test input");
        assert_eq!(front, 2, "Part two test input");
    }

    #[test]
    fn day9_both_real_input() {
        let (front, back) = predict_sum_ends(parse_input(super::INPUT));
        assert_eq!(back, 1782868781, "Part one real input");
        assert_eq!(front, 1057, "Part two real input");
    }
}
