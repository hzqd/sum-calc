use std::process;
use aoko::{standard::functions::{ext::StdAnyExt, fun::read_line}, no_std::pipelines::tap::Tap};
fn main() {
    loop {
        read_line()
            .tap(|s| if s.is_empty() { process::exit(0) })
            .split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok())
            .sum::<f64>()
            .echo();
    }
}