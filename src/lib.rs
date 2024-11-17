use std::{collections::HashMap, fmt::Display, hash::BuildHasher, time::{Duration, Instant}};

use colored::Colorize;
use puzzles::Scenario;
use solving::Status;

mod arguments;
mod solving;
mod puzzles;
mod utils;
mod manifest;

pub use {
    puzzles::{Puzzle, Day},
    solving::{Solver, SolverResult},
    arguments::{ExecutionOptions, Scope},
    manifest::{Manifest, DataManifest, DayManifest, PuzzleManifest, Example, locate_manifests}
};

pub fn execute<E: Display, H: BuildHasher + Sync>(options: ExecutionOptions, manifest: &Manifest<E, H>) {
    let scenarios  = options.scope.scenarios(&manifest.data, options.examples);
    println!("Executing {} scenarios(s)...", scenarios.len());

    let start_time = Instant::now();
    let results = execute_scenarios(options, &scenarios, manifest);

    let mut stats = HashMap::new();
    for result in results {
        *stats.entry(result).or_default() += 1;
    }

    let duration = start_time.elapsed();
    print_summary(&stats, duration);
}

#[cfg(feature = "parallel")]
fn execute_scenarios<E: Display, H: BuildHasher + Sync>(
    options: ExecutionOptions,
    scenarios: &[Scenario],
    manifest: &Manifest<E, H>
) -> Vec<Status> {
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

    scenarios
        .par_iter()
        .map(|scenario| scenario.execute(options, manifest))
        .collect::<Vec<_>>()
}

#[cfg(not(feature = "parallel"))]
fn execute_scenarios<E: Display, H: BuildHasher>(
    options: ExecutionOptions,
    scenarios: &[Scenario],
    manifest: &Manifest<E, H>
) -> Vec<Status> {
    scenarios
        .iter()
        .map(|scenario| scenario.execute(options, manifest))
        .collect::<Vec<_>>()
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
        let msg = format!("{failed} / {total_puzzles} {puzzles} failed").bold().bright_red();
        println!("{msg}");
    }

    if not_ran > 0 {
        let msg = format!("{not_ran} / {total_puzzles} {puzzles} were skipped").bold().bright_yellow();
        println!("{msg}");
    }
}