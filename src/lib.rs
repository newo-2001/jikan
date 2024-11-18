use std::{collections::HashMap, fmt::Display, hash::BuildHasher, time::{Duration, Instant}};

use colored::Colorize;
use itertools::Itertools;
use puzzles::scenarios_for_puzzle;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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
    manifest::{DayManifest, PuzzleManifest, Example, ManifestProvider}
};

pub fn execute<P: ManifestProvider, E: Display, H: BuildHasher + Sync>(options: ExecutionOptions, solvers: &HashMap<Puzzle, Solver<E>, H>) {
    let message = "Executing scenarios...".bold().blue();
    println!("{message}");

    let days: Vec<Day> = solvers
        .keys()
        .copied()
        .map(Into::<Day>::into)
        .filter(|&day| options.scope.contains_day(day))
        .unique()
        .collect();

    let start_time = Instant::now();
    let results = execute_days::<E, H, P>(options, &days, solvers);

    let mut stats = HashMap::new();
    for result in results {
        *stats.entry(result).or_default() += 1;
    }

    let duration = start_time.elapsed();
    print_summary(&stats, duration);
}

fn execute_days<E: Display, H: BuildHasher + Sync, P: ManifestProvider>(
    options: ExecutionOptions,
    days: &[Day],
    solvers: &HashMap<Puzzle, Solver<E>, H>
) -> Vec<Status> {
    days
        .par_iter()
        .filter_map(|&day| {
            match P::get_manifest(day) {
                Ok(manifest) => Some(execute_day(day, solvers, &manifest, options)),
                Err(err) => {
                    let err = format!("Failed to retrieve manifest for day {day}: {}", err.to_string().red());
                    println!("{err}");
                    None
                }
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn execute_day<E:Display, H: BuildHasher + Sync>(
    day: Day,
    solvers: &HashMap<Puzzle, Solver<E>, H>,
    manifest: &DayManifest,
    options: ExecutionOptions
) -> Vec<Status> {
    manifest.parts
        .iter()
        .enumerate()
        .filter_map(|(part, manifest)| {
            let puzzle = Puzzle { year: day.year, day: day.day, part: part + 1 };

            options.scope
                .contains_puzzle(puzzle)
                .then_some((puzzle, manifest))
        })
        .flat_map(|(puzzle, manifest)|
            scenarios_for_puzzle(puzzle, manifest, options.examples)
        )
        .map(|scenario| scenario.execute(options, manifest, solvers))
        .collect()
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