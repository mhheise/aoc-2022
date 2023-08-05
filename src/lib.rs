use color_eyre::Result;

use std::{env, fs};

/// Read the input file for a given day.
pub fn read_file(folder: &str, day: u8) -> Result<String> {
    let cwd = env::current_dir()?;
    let filepath = cwd.join(format!("src/{folder}/{:02}.txt", day));
    let file = fs::read_to_string(filepath)?;

    Ok(file)
}

/// Extract the solver time from the output of a given day.
pub fn extract_solver_time(output: &str) -> Result<u32> {
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
        use std::fmt::{Debug, Display};
        use std::time::{Duration, Instant};

        fn format_solver_time(elapsed: Duration) -> String {
            if elapsed.as_millis() > 0 {
                format!("{}ms", elapsed.as_millis())
            } else {
                format!("{}μs", elapsed.as_micros())
            }
        }

        fn timed_solution<T: Debug + Display>(
            input: &str,
            fn1: impl FnOnce(&str) -> Option<T>,
            fn2: impl FnOnce(&str) -> Option<T>,
        ) {
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
