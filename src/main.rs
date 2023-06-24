use std::{process, ops::Not};
use aoko::{standard::functions::fun::read_line, no_std::{pipelines::{tap::Tap, pipe::Pipe}, cps::operators::Op}};
fn main() {
    loop {
        read_line()
            .tap(|s| if s.trim_end().is_empty() { process::exit(0) })
            .split(|c: char| c.is_ascii_digit().not() && c != '.' && c != '-')
            .flat_map(|s| s.starts_with('-').not_c(|b| s.split(move |c| b && c == '-')))
            .filter(|s| s.is_empty().not())
            .try_fold(0.0, |acc, s| s.parse().map_or(Err(s), |n: f64| Ok(acc + n)))
            .pipe(|r| match r {
                Ok(v) => println!("{v}"),
                Err(e) => println!("Extract numeric error from '{e}'")
            });
    }
}