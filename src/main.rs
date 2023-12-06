use std::env::args;

mod day_01;
mod day_02;
mod day_03;

fn main() {
    let day: Option<u8> = args().nth(1).and_then(|s| s.parse().ok());

    let fns = [day_01::run, day_02::run, day_03::run];
    if let Some(day) = day {
        let result = match day {
            1 => day_01::run(),
            2 => day_02::run(),
            3 => day_03::run(),
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
