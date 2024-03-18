use std::fmt::Display;

use num::integer::gcd;

pub fn first(part_one: impl Display) -> String {
    format!("Part one:\t{part_one}")
}

pub fn both(part_one: impl Display, part_two: impl Display) -> String {
    format!("Part one:\t{part_one}\nPart two:\t{part_two}")
}

pub fn lcm(xs: &[usize]) -> usize {
    if xs.len() == 1 {
        return xs[0];
    }

    let a = xs[0];
    let b = lcm(&xs[1..]);
    a * b / gcd(a, b)
}
