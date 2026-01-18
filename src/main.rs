/// Limited cage pair combinations solver.
mod engine_v1;

/// Experimental full sudoku solver.
mod engine_v2;

use clap::Parser;
use engine_v2::{SolveResult, State, Sudoku, solve_sudoku};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "sum-sequence-sudoku-solver")]
#[command(about = "A Sudoku solver for a custom puzzle rule set", long_about = None)]
struct Args {
    /// Engine version to use (1 or 2)
    #[arg(short, long, default_value_t = 2)]
    engine: u8,

    /// Use extended sum-sequence rules
    #[arg(short, long, default_value_t = false)]
    sum_sequence: bool,

    /// Path to sudoku input file [default: ./sudoku.txt]
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Iteration limit for engine v2
    #[arg(short, long, default_value_t = 2_000_000)]
    limit: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.engine {
        1 => engine_v1::generate()?,
        2 => run_engine_v2(args.file, args.sum_sequence, args.limit)?,
        _ => return Err(format!("Invalid engine: {}. Use '1' or '2'", args.engine).into()),
    }

    Ok(())
}

fn run_engine_v2(file_path: Option<PathBuf>, sum_sequence: bool, limit: usize) -> Result<(), Box<dyn Error>> {
    let path = file_path.unwrap_or_else(|| PathBuf::from("sudoku.txt"));

    let sudoku = if path.exists() {
        parse_sudoku_file(&path)?
    } else {
        return Err(format!("File not found: {}", path.display()).into());
    };

    println!("Input Sudoku:");
    println!("{}", sudoku);

    let start = std::time::Instant::now();
    let result = solve_sudoku(sudoku, sum_sequence, limit);
    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);

    let SolveResult(counter, sudoku, state) = result;

    match state {
        State::Solved => println!("Solved in {} iterations:", counter),
        State::Stuck => eprintln!("No progress made after {} iterations", counter),
        State::LimitReached => eprintln!("Reached iteration limit: {}", counter),
        State::Unsolvable => return Err("Sudoku is unsolvable:".into()),
    }

    println!("{}", sudoku);

    Ok(())
}

fn parse_sudoku_file(path: &PathBuf) -> Result<Sudoku, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().filter(|line| !line.trim().is_empty()).collect();

    if lines.len() != 9 {
        return Err(format!("Invalid sudoku format: expected 9 rows, got {}", lines.len()).into());
    }

    let mut grid = [[0u16; 9]; 9];
    for (i, line) in lines.iter().enumerate() {
        let numbers: Vec<&str> =
            line.split(|c: char| c == ',' || c.is_whitespace()).filter(|s| !s.is_empty()).collect();

        if numbers.len() != 9 {
            return Err(
                format!("Invalid sudoku format: row {} has {} numbers, expected 9", i + 1, numbers.len()).into()
            );
        }

        for (j, num_str) in numbers.iter().enumerate() {
            match num_str.trim().parse::<u16>() {
                Ok(num) if num <= 9 => grid[i][j] = num,
                _ => {
                    return Err(format!("Invalid number '{}' at row {}, col {}", num_str, i + 1, j + 1).into());
                }
            }
        }
    }

    Ok(Sudoku(grid))
}
