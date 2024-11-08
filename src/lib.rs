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
    puzzles::Puzzle,
    solving::{Solver, SolverResult},
    arguments::{ExecutionOptions, Scope},
    manifest::{Manifest, DataManifest, DayManifest, PuzzleManifest, Example}
};

pub fn execute<E: Display, H: BuildHasher + Sync>(options: ExecutionOptions, manifest: &Manifest<E, H>) {
    let scenarios  = options.scope.scenarios(&manifest.data, options.examples);
    println!("Executing {} scenarios(s)...", scenarios.len());

    let start_time = Instant::now();

    let results = scenarios
        .par_iter()
        .map(|&puzzle| {
            let result = if options.verify {
                puzzle.verify(manifest)
            } else {
                puzzle.run(manifest)
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
    let puzzles = if total_puzzles == 1 { "scenario" } else { "scenarios" };
    
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