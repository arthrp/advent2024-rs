use std::{env, fs};

use day3::Day3Solver;

mod day1;
mod day2;
mod day3;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1])?;

    let solver = Day3Solver::new(content);
    let result = solver.solve_additional();

    println!("Result: {}", result);

    Ok(())
}
