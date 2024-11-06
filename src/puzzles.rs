use std::{fmt::{Display, Formatter}, fs};

use serde::Deserialize;
use thiserror::Error;

use crate::{Scope, SolverProvider};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Puzzle {
    pub year: u32,
    pub day: u32,
    pub part: u32
}

#[derive(Debug, Deserialize)]
pub (crate) struct PuzzleData {
    pub input: String,
    pub solutions: Vec<String>
}

#[derive(Debug, Error)]
pub (crate) enum DataError {
    #[error("Failed to resolve input file: {0}")]
    NoFile(String),
    #[error("Failed to parse solution file: {0}")]
    Malformed(#[from] serde_yml::modules::error::Error)
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} day {:02} part {}", self.year, self.day, self.part)
    }
}

impl Puzzle {
    pub (crate) fn get_data(self) -> Result<PuzzleData, DataError> {
        let path = format!("data/{}/day_{:02}.yaml", self.year, self.day);
        let content = fs::read_to_string(&path)
            .map_err(|_| DataError::NoFile(path))?;

        Ok(serde_yml::from_str::<PuzzleData>(&content)?)
    }
}

impl Scope {
    pub (crate) fn puzzles(self, provider: &impl SolverProvider) -> Vec<Puzzle> {
        match self {
            Scope::All => provider.solvers()
                .keys()
                .copied()
                .collect(),
            Scope::Year(year) => provider.solvers()
                .keys()
                .filter(|puzzle| puzzle.year == year)
                .copied()
                .collect(),
            Scope::Day(year, day) => provider.solvers()
                .keys()
                .filter(|puzzle| puzzle.year == year && puzzle.day == day)
                .copied()
                .collect(),
            Scope::Puzzle(puzzle) => vec![puzzle]
        }
    }
}