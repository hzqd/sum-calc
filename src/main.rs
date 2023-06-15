use std::process;
use aoko::{standard::functions::{ext::StdAnyExt, fun::read_line}, no_std::pipelines::tap::Tap};
use regex::Regex;

fn main() {
    let r = Regex::new(r"-?[0-9]*\.?[0-9]+").unwrap();

    loop {
        read_line()
            .tap(|s| if s.trim_end().is_empty() { process::exit(0) })
            .split_whitespace()
            .flat_map(|s| r.find_iter(s))
            .filter_map(|m| m.as_str().parse::<f64>().ok())
            .sum::<f64>()
            .echo();
    }
}
