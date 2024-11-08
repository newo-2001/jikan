use std::{collections::HashMap, fmt::Display, hash::BuildHasher, time::{Duration, Instant}};

use colored::Colorize;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use solving::Status;

mod arguments;
mod solving;
mod puzzles;
mod utils;
mod manifest;

pub use {
    puzzles::{Puzzle, Day},
    solving::{Solver, SolverResult},
    arguments::{Action, Scope, Error},
    manifest::{Manifest, DataManifest, DayManifest, PuzzleManifest, TestCase}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionOptions {
    scope: Scope,
    action: Action
}

pub fn execute<E: Display, H: BuildHasher + Sync>(options: ExecutionOptions, manifest: &Manifest<E, H>) {
    let puzzles = options.scope.puzzles(&manifest.data);
    println!("Executing {} puzzle(s)...", puzzles.len());

    let start_time = Instant::now();

    let results = puzzles
        .par_iter()
        .map(|&puzzle| {
            let result = match options.action {
                Action::Run => puzzle.run(manifest),
                Action::Verify => puzzle.verify(manifest)
            };

            let puzzle_str = format!("[{puzzle}]").bold().bright_blue();
            println!("{puzzle_str} {result}");

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