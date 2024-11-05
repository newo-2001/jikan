#![allow(
    clippy::module_name_repetitions,
    clippy::similar_names
)]

use std::{time::{Instant, Duration}, collections::HashMap};

use colored::Colorize;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use solver::Status;

mod arguments;
mod solver;
mod puzzle;
mod utils;

pub use {
    puzzle::Puzzle,
    solver::{Solver, SolverResult},
    arguments::{Action, Scope, Error}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionOptions {
    scope: Scope,
    action: Action
}

pub fn execute(options: ExecutionOptions, provider: &impl SolverProvider) {
    let puzzles = options.scope.puzzles(provider);

    println!("Executing {} puzzle(s)...", puzzles.len());

    let start_time = Instant::now();

    let results = puzzles
        .par_iter()
        .map(|&puzzle| {
            let result = match options.action {
                Action::Run => puzzle.run(provider),
                Action::Verify => puzzle.verify(provider)
            };

            let puzzle_str = format!("[{puzzle}]").bold().bright_blue();
            println!("{puzzle_str}{result}");

            result.status()
        }).collect::<Vec<_>>();

    let mut stats = HashMap::new();
    for result in results {
        *stats.entry(result).or_default() += 1;
    }

    let duration = start_time.elapsed();
    print_summary(&stats, duration);
}


// TODO: Clean this up
fn print_summary(stats: &HashMap<Status, usize>, duration: Duration) {
    let [succeeded, not_ran, failed] = [Status::Success, Status::Skipped, Status::Failure]
        .map(|status| *stats.get(&status).unwrap_or(&0));

    let total_puzzles: usize = stats.values().sum();
    let puzzles = if total_puzzles == 1 { "puzzle" } else { "puzzles" };
    
    let msg = format!("Execution took {}", utils::format_duration(&duration)).bold().bright_blue();
    println!("\n{msg}");

    if succeeded > 0 {
        let msg = format!("{succeeded} / {total_puzzles} {puzzles} executed successfully").bold().bright_green();
        println!("{msg}");
    }

    if failed > 0 {
        let msg = format!("{failed} / {total_puzzles} {puzzles} failed to execute").bold().bright_red();
        println!("{msg}");
    }

    if not_ran > 0 {
        let msg = format!("{not_ran} / {total_puzzles} {puzzles} were not executed").bold().bright_yellow();
        println!("{msg}");
    }
}

pub trait SolverProvider: Sync {
    fn get_solver(&self, puzzle: Puzzle) -> Option<Box<Solver>>;
    fn manifest(&self) -> impl IntoIterator<Item=Puzzle>;
}