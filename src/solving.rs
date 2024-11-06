use std::{collections::HashMap, fmt::Display, hash::BuildHasher, time::{Duration, Instant}};

use colored::{ColoredString, Colorize};
use thiserror::Error;

use crate::{puzzles::DataError, utils, Puzzle};

#[derive(Debug, Clone, Error)]
pub (crate) enum Error {
    #[error("Solver produced an incorrect answer, expected: `{expected}` got `{actual}`")]
    IncorrectAnswer {
        expected: String,
        actual: String
    },
    #[error("An error occurred whilst executing the puzzle:\n\t{0}")]
    ExecutionError(String),
}

#[derive(Debug, Error)]
pub (crate) enum ResolutionError {
    #[error("Failed to resolve solver")]
    Solver,
    #[error("Failed to resolve solution to part {0}")]
    Solution(u32),
    #[error(transparent)]
    Data(#[from] DataError)
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
pub (crate) enum Result {
    Success {
        result: String,
        stats: Statistics,
    },
    Failure {
        error: Error,
        stats: Statistics
    },
    Skipped(ResolutionError),
}

impl Result {
    pub fn status(&self) -> Status {
        match *self {
            Self::Success { .. } => Status::Success,
            Self::Failure { .. } => Status::Failure,
            Self::Skipped { .. } => Status::Skipped
        }
    }
}

impl Display for Result {
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

impl Puzzle {
    pub (crate) fn run<H: BuildHasher>(self, provider: &HashMap<Puzzle, Solver, H>) -> Result {
        let solver = match provider.get(&self) {
            None => return Result::Skipped(ResolutionError::Solver),
            Some(solver) => solver,
        };

        let data = match self.get_data() {
            Err(err) => return Result::Skipped(ResolutionError::Data(err)),
            Ok(result) => result
        };

        let start_time = Instant::now();
        let result = match solver(&data.input) {
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

    pub (crate) fn verify<H: BuildHasher>(self, provider: &HashMap<Puzzle, Solver, H>) -> Result {
        let data = match self.get_data() {
            Err(err) => return Result::Skipped(ResolutionError::Data(err)),
            Ok(result) => result
        };

        let (result, stats) = match self.run(provider) {
            err @ (Result::Skipped(_) | Result::Failure { .. }) => return err,
            Result::Success { result, stats } => (result, stats)
        };

        let expected = match data.solutions.get(self.part as usize) {
            None => return Result::Skipped(ResolutionError::Solution(self.part)),
            Some(solution) => solution
        };

        if expected == &result {
            Result::Success { result, stats }
        } else {
            Result::Failure {
                error: Error::IncorrectAnswer {
                    expected: expected.clone(),
                    actual: result
                },
                stats
            }
        }
    }
}

pub type Solver = fn(&str) -> SolverResult;
pub type SolverResult = std::result::Result<Box<dyn Display + Sync>, Box<dyn std::error::Error + Sync>>;