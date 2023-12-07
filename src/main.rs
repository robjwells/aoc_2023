use std::env::args;

use aoc_2023::{day01, day02, day03, day04, day05};

fn main() {
    let day: Option<u8> = args().nth(1).and_then(|s| s.parse().ok());

    let fns = [day01, day02, day03, day04, day05];
    if let Some(day) = day {
        let result = match day {
            1 => day01(),
            2 => day02(),
            3 => day03(),
            4 => day04(),
            5 => day05(),
            _ => {
                format!("Day {} not implemented yet.", day)
            }
        };
        println!("{}", result)
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
