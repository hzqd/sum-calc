use std::process;
use aoko::{standard::functions::{ext::StdAnyExt, fun::read_line}, no_std::pipelines::tap::Tap};
fn main() {
    loop {
        read_line()
            .tap(|s| if s.trim_end().is_empty() { process::exit(0) })
            .split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
            .flat_map(|s| if !s.starts_with('-') && s.contains('-') { s.split('-').collect() } else { vec![s] })
            .filter_map(|s| s.parse::<f64>().ok())
            .sum::<f64>()
            .echo();
    }
}