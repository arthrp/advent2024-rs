use std::{env, fs};

use day1::Day1Solver;

mod day1;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1])?;

    let solver = Day1Solver::new(content);
    let result = solver.solve_additional();

    println!("Result: {}", result);

    Ok(())
}
