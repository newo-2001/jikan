use std::collections::HashMap;

use serde::Deserialize;

use crate::{puzzles::Day, solving::ResolutionError, Puzzle, Solver};

pub struct Manifest<E, H> {
    pub solvers: HashMap<Puzzle, Solver<E>, H>,
    pub data: DataManifest
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataManifest {
    pub puzzles: HashMap<Day, DayManifest>
}

#[derive(Debug, Clone, Deserialize)]
pub struct DayManifest {
    #[serde(default)]
    pub parts: Vec<PuzzleManifest>,
    #[serde(default)]
    pub input: Option<String>
}

#[derive(Debug, Clone, Deserialize)]
pub struct PuzzleManifest {
    #[serde(default)]
    pub examples: Vec<TestCase<String>>,
    #[serde(default)]
    pub input: Option<String>,
    pub solution: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestCase<T> {
    pub input: T,
    pub solution: T
}

impl DataManifest {
    pub (crate) fn for_puzzle(&self, puzzle: Puzzle) -> Result<TestCase<&str>, ResolutionError> {
        let day = self.puzzles.get(&puzzle.get_day())
            .ok_or(ResolutionError::Data)?;

        let part = day.parts.get(puzzle.part as usize - 1)
            .ok_or(ResolutionError::Data)?;

        let input = part.input.as_deref()
            .or(day.input.as_deref())
            .ok_or(ResolutionError::Input)?;

        Ok(TestCase { solution: &part.solution, input })
    }
}