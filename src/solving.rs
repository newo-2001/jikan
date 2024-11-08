use std::{fmt::Display, hash::BuildHasher, time::{Duration, Instant}};

use colored::{ColoredString, Colorize};
use thiserror::Error;

use crate::{puzzles::Scenario, utils, Manifest};

pub (crate) struct ScenarioData<'a> {
    pub (crate) input: &'a str,
    pub (crate) solution: Option<&'a str>
}

#[derive(Debug, Clone, Error)]
pub (crate) enum Error<'a> {
    #[error("Solver produced an incorrect answer, expected: `{expected}` got `{actual}`")]
    IncorrectAnswer {
        expected: &'a str,
        actual: String
    },
    #[error("An error occurred whilst executing the puzzle:\n\t{0}")]
    ExecutionError(String),
}

#[derive(Debug, Error)]
pub (crate) enum ResolutionError {
    #[error("Failed to resolve solver")]
    Solver,
    #[error("Puzzle not found in data manifest")]
    Puzzle,
    #[error("No input found for puzzle in data manifest")]
    Input,
    #[error("No solution found for puzzle in data manifest")]
    Solution
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub (crate) enum Status {
    Success,
    Failure,
    Skipped
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub (crate) struct Statistics {
    pub duration: Duration
}

#[derive(Debug)]
pub (crate) enum Result<'a> {
    Success {
        result: String,
        stats: Statistics,
    },
    Failure {
        error: Error<'a>,
        stats: Statistics
    },
    Skipped(ResolutionError),
}

impl Result<'_> {
    pub fn status(&self) -> Status {
        match *self {
            Self::Success { .. } => Status::Success,
            Self::Failure { .. } => Status::Failure,
            Self::Skipped { .. } => Status::Skipped
        }
    }
}

impl Display for Result<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct DisplayInfo {
            status: ColoredString,
            output: ColoredString,
            duration: Option<Duration>
        }

        let info = match self {
            Self::Success { stats, result } => DisplayInfo {
                status: "(Success)".bold().bright_green(),
                output: result.to_string().bright_green(),
                duration: Some(stats.duration)
            },
            Self::Failure { stats, error } => DisplayInfo {
                status: "(Failure)".bold().bright_red(),
                output: error.to_string().bright_red(),
                duration: Some(stats.duration)
            },
            Self::Skipped(error) => DisplayInfo {
                status: "(Not Run)".bold().bright_yellow(),
                output: error.to_string().bright_yellow(),
                duration: None
            }
        };

        match info {
            DisplayInfo { duration: Some(duration), status, output } => {
                write!(f, "{} {} {}", utils::format_duration(&duration), status, output)
            },
            DisplayInfo { duration: None, status, output } => {
                write!(f, "{status} {output}")
            }
        }
    }
}

impl Scenario {
    pub (crate) fn run<E: Display, H: BuildHasher>(self, manifest: &Manifest<E, H>) -> Result {
        let puzzle = match self {
            | Scenario::Example { puzzle, .. }
            | Scenario::Puzzle(puzzle) => puzzle
        };

        let solver = match manifest.solvers.get(&puzzle) {
            None => return Result::Skipped(ResolutionError::Solver),
            Some(solver) => solver,
        };

        let data = match manifest.data.for_scenario(self) {
            Err(err) => return Result::Skipped(err),
            Ok(result) => result
        };

        let start_time = Instant::now();
        let result = match solver(data.input) {
            Err(error) => return Result::Failure {
                stats: Statistics {
                    duration: start_time.elapsed()
                },
                error: Error::ExecutionError(error.to_string())
            },
            Ok(result) => result.to_string()
        };

        Result::Success {
            stats: Statistics {
                duration: start_time.elapsed()
            },
            result
        }
    }

    pub (crate) fn verify<E: Display, H: BuildHasher>(self, manifest: &Manifest<E, H>) -> Result {
        let data = match manifest.data.for_scenario(self) {
            Err(err) => return Result::Skipped(err),
            Ok(result) => result
        };

        let solution = match data.solution {
            None => return Result::Skipped(ResolutionError::Solution),
            Some(solution) => solution
        };

        let (result, stats) = match self.run(manifest) {
            err @ (Result::Skipped(_) | Result::Failure { .. }) => return err,
            Result::Success { result, stats } => (result, stats)
        };

        if solution == result {
            Result::Success { result, stats }
        } else {
            Result::Failure {
                error: Error::IncorrectAnswer {
                    expected: solution,
                    actual: result
                },
                stats
            }
        }
    }
}

pub type Solver<E = Box<dyn std::error::Error>> = fn(&str) -> SolverResult<E>;
pub type SolverResult<E = Box<dyn std::error::Error>> = std::result::Result<Box<dyn Display>, E>;