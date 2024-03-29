#![doc = include_str!("../README.md")]
#![feature(test)]

use std::{env, fs};

use anyhow::Result;

/// Module for common imports.
///
/// This prelude is designed for _convenience_ rather than performance. While it is possible to
/// specify individual imports, this module is meant to be used as a wildcard import to
/// facilitate rapid prototyping.
///
/// # Examples
///
/// ```rust
/// use aoc::prelude::*;
///
/// let input = read_file("inputs", 0).unwrap();
/// for line in input.lines() {
///     println!("{}", input);
/// }
/// ```
pub mod prelude;

/// Module for test-specific imports.
///
/// This prelude re-exports utilities for prettier assertions, parameterized tests, and nightly
/// features for running micro-benchmarks.
///
/// # Examples
///
/// ```rust
/// #![feature(test)]
///
/// #[cfg(test)]
/// mod tests {
///     use aoc::testing::*;
///
///     #[test]
///     fn test() {
///         assert_eqp!(1, 1);
///     }
///
///     #[bench]
///     fn bench(b: &mut Bencher) {
///         b.iter(|| 1 + 1);
///     }
/// }
/// ```
pub mod testing;

/// Module for common utilities.
pub mod utils;

/// Read a text file given a folder name and day number.
///
/// This function assumes that the file is located at `src/{folder}/{day:02}.txt`;
/// for example, the input file for day 1 would be located at `src/inputs/01.txt`.
///
/// # Errors
///
/// [`std::io::Error`] if any of the following occur:
///   - The current working directory is invalid
///   - The file cannot be read
pub fn read_file(folder: &str, day: u8) -> Result<String> {
    let cwd = env::current_dir()?;
    let filepath = cwd.join(format!("src/{folder}/{day:02}.txt"));
    let file = fs::read_to_string(filepath)?;

    Ok(file)
}

/// Extract the solver time from the output of a given day.
///
/// # Panics
///
/// This function will panic if the output is empty.
///
/// # Errors
///
/// This function will return an error if the output cannot be parsed.
pub fn solver_time(output: &str) -> Result<u32> {
    let binding = output.trim().split(' ').collect::<Vec<&str>>();
    let out = binding.last().unwrap();

    let time = if out.ends_with("ms") {
        out[0..out.len() - 2].parse::<u32>()? * 1000
    } else {
        out[0..out.len() - 3].parse::<u32>()?
    };

    Ok(time)
}

/// Macro to solve a given day.
///
/// # Example
///
/// ```rust
/// use aoc::{read_file, solve};
///
/// let input = "abc";
///
/// fn p1(input: &str) -> Option<u32> {
///     Some(1)
/// }
///
/// fn p2(input: &str) -> Option<u32> {
///     Some(2)
/// }
///
/// solve!(input, p1, p2)
/// ```
#[macro_export]
macro_rules! solve {
    ($input:expr, $p1:ident, $p2:ident) => {{
        use std::{
            fmt::{Debug, Display},
            time::{Duration, Instant},
        };

        fn format_solver_time(elapsed: Duration) -> String {
            if elapsed.as_millis() > 0 {
                format!("{}ms", elapsed.as_millis())
            } else {
                format!("{}μs", elapsed.as_micros())
            }
        }

        fn timed_solution<'a, T, U>(
            input: &'a str,
            fn1: impl FnOnce(&'a str) -> Option<T>,
            fn2: impl FnOnce(&'a str) -> Option<U>,
        ) where
            T: Debug + Display,
            U: Debug + Display,
        {
            let timer = Instant::now();
            let r1 = fn1(input);
            let r2 = fn2(input);
            let elapsed = timer.elapsed();
            let time = format_solver_time(elapsed);

            match (r1, r2) {
                (Some(r1), Some(r2)) => println!("{r1} {r2} {time}"),
                (Some(r1), None) => println!("{r1} * {time}"),
                (None, Some(r2)) => println!("* {r2} {time}"),
                (None, None) => println!("* * {time}"),
            }
        }

        timed_solution($input, $p1, $p2);
    }};
}
