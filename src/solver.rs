use std::{fmt::Display, time::{Duration, Instant}};

use colored::{ColoredString, Colorize};
use thiserror::Error;

use crate::{utils, Puzzle, SolverProvider};

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Solver produced an incorrect answer, expected: `{expected}` got `{actual}`")]
    IncorrectAnswer {
        expected: String,
        actual: String
    },
    #[error("An error occurred whilst executing the puzzle:\n\t{0}")]
    ExecutionError(String),
}

#[derive(Debug, Clone, Error)]
pub (crate) enum ResolutionError {
    #[error("Failed to resolve solver")]
    Solver,
    #[error("Failed to resolve input file: {0}")]
    InputFile(String),
    #[error("Failed to resolve solution file: {0}")]
    SolutionFile(String),
    #[error("Failed to locate solution entry")]
    SolutionEntry
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

#[derive(Debug, Clone)]
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
    pub (crate) fn run(self, provider: &impl SolverProvider) -> Result {
        let solver = match provider.get_solver(self) {
            None => return Result::Skipped(ResolutionError::Solver),
            Some(solver) => solver,
        };

        let input = match self.get_input() {
            Err(error) => return Result::Skipped(error),
            Ok(input) => input,
        };

        let start_time = Instant::now();
        let result = match solver(&input) {
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

    pub (crate) fn verify(self, provider: &impl SolverProvider) -> Result {
        let expected = match self.get_solution() {
            Ok(solution) => solution.replace("\r\n", "").replace('\n', ""),
            Err(err) => return Result::Skipped(err)
        };

        let (result, stats) = match self.run(provider) {
            err @ (Result::Skipped(_) | Result::Failure { .. }) => return err,
            Result::Success { result, stats } => (result, stats)
        };

        let actual = result.replace("\r\n", "").replace('\n', "");

        if expected == actual {
            Result::Success { result, stats }
        } else {
            Result::Failure {
                error: Error::IncorrectAnswer { expected, actual },
                stats
            }
        }
    }
}

pub type Solver = dyn Fn(&str) -> SolverResult;
pub type SolverResult<'a> = std::result::Result<Box<dyn Display + Sync + 'a>, Box<dyn std::error::Error + Sync + 'a>>;