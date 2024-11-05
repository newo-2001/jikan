use std::{fmt::{Display, Formatter}, fs};

use crate::{solver::ResolutionError, Scope, SolverProvider};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Puzzle {
    pub year: u32,
    pub day: u32,
    pub part: u32
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} day {:02} part {}", self.year, self.day, self.part)
    }
}

impl Puzzle {
    pub (crate) fn get_input(self) -> Result<String, ResolutionError> {
        let path = format!("inputs/{}/day_{:02}.txt", self.year, self.day);
        fs::read_to_string(&path)
            .map_err(|_| ResolutionError::InputFile(path))
    }

    pub (crate) fn get_solution(self) -> Result<String, ResolutionError> {
        let path = format!("solutions/{}/day_{:02}.txt", self.year, self.day);
        let content = fs::read_to_string(&path)
            .map_err(|_| ResolutionError::SolutionFile(path))?;

        let solution = *content.chars()
            .as_str()
            .split(';')
            .collect::<Vec<_>>()
            .get(self.part as usize - 1)
            .ok_or(ResolutionError::SolutionEntry)?;

        if solution.is_empty() { Err(ResolutionError::SolutionEntry) }
        else { Ok(solution.to_owned()) }
    }
}

impl Scope {
    pub (crate) fn puzzles(self, provider: &impl SolverProvider) -> Vec<Puzzle> {
        match self {
            Scope::All => provider.manifest()
                .into_iter()
                .collect(),
            Scope::Year(year) => provider.manifest()
                .into_iter()
                .filter(|puzzle| puzzle.year == year)
                .collect(),
            Scope::Day(year, day) => provider.manifest()
                .into_iter()
                .filter(|puzzle| puzzle.year == year && puzzle.day == day)
                .collect(),
            Scope::Puzzle(puzzle) => vec![puzzle]
        }
    }
}