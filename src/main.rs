use std::{env, fs};

use day2::Day2Solver;
use day3::Day3Solver;

mod day1;
mod day2;
mod day3;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1])?;

    // let solver = Day2Solver::new(content);
    // let result = solver.solve(true);

    let solver = Day3Solver::new(content);
    let result = solver.solve();

    println!("Result: {}", result);

    Ok(())
}
