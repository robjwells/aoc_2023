use std::fmt::Display;
pub fn first(part_one: impl Display) -> String {
    format!("Part one:\t{part_one}")
}

pub fn both(part_one: impl Display, part_two: impl Display) -> String {
    format!("Part one:\t{part_one}\nPart two:\t{part_two}")
}
