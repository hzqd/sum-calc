use std::{process, ops::Not};
use aoko::{standard::functions::fun::read_line, no_std::pipelines::{tap::Tap, pipe::Pipe}};
use rayon::{str::ParallelString, prelude::ParallelIterator};
fn main() {
    loop {
        read_line()
            .tap(|s| if s.trim_end().is_empty() { process::exit(0) })
            .par_split(|c: char| c.is_ascii_digit().not() && c != '.' && c != '-')
            .filter(|s| s.is_empty().not())
            .map(|s| s.parse().map_err(|_| s))
            .try_reduce(|| 0.0, |a, n| Ok(a + n))
            .pipe(|r| match r {
                Ok(v) => println!("{v}"),
                Err(e) => println!("Extract numeric error from '{e}'")
            });
    }
}