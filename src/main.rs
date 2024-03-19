use std::env::args;

use aoc_2023::*;

fn main() {
    let day: Option<usize> = args().nth(1).and_then(|s| s.parse().ok());

    let fns = [
        day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    ];
    if let Some(day) = day {
        if let Some(day_func) = fns.get(day - 1) {
            println!("{}", day_func());
        } else {
            eprintln!("Day {} not implemented yet.", day)
        }
    } else {
        for (idx, func) in fns.iter().enumerate() {
            if idx != 0 {
                println!();
            }
            println!("Day {}", idx + 1);
            println!("{}", "=".repeat(32));
            println!("{}", func());
        }
    }
}
