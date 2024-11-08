use std::collections::HashMap;

use serde::Deserialize;

use crate::{puzzles::{Day, Scenario}, solving::{ResolutionError, ScenarioData}, Puzzle, Solver};

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
    pub examples: Vec<Example>,
    #[serde(default)]
    pub input: Option<String>,
    pub solution: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Example {
    pub input: String,
    pub solution: Option<String>
}

impl DataManifest {
    pub (crate) fn for_scenario(&self, scenario: Scenario) -> Result<ScenarioData, ResolutionError> {
        let puzzle = match scenario {
            | Scenario::Puzzle(puzzle)
            | Scenario::Example { puzzle, .. } => puzzle
        };

        let day = self.puzzles.get(&Day { year: puzzle.year, day: puzzle.day })
            .ok_or(ResolutionError::Puzzle)?;

        let part = day.parts.get(puzzle.part - 1)
            .ok_or(ResolutionError::Puzzle)?;

        match scenario {
            Scenario::Puzzle { .. } => {
                let input = part.input.as_deref()
                    .or(day.input.as_deref())
                    .ok_or(ResolutionError::Input)?;
            
                Ok(ScenarioData { solution: part.solution.as_deref(), input })
            },
            Scenario::Example { number, .. } => {
                let example = &part.examples[number - 1];
                Ok(ScenarioData {
                    solution: example.solution.as_deref(),
                    input: &example.input
                })
            }
        }
    }
}