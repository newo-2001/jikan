use std::error::Error;

use serde::Deserialize;

use crate::{puzzles::{Day, Scenario}, solving::{ResolutionError, ScenarioData}};

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

impl DayManifest {
    pub (crate) fn data_for_scenario(&self, scenario: Scenario) -> Result<ScenarioData<'_>, ResolutionError> {
        let puzzle = match scenario {
            | Scenario::Puzzle(puzzle)
            | Scenario::Example { puzzle, .. } => puzzle
        };

        let part = self.parts.get(puzzle.part - 1).unwrap();

        match scenario {
            Scenario::Puzzle { .. } => {
                let input = part.input.as_deref()
                    .or(self.input.as_deref())
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

pub trait ManifestProvider {
    fn get_manifest(day: Day) -> Result<DayManifest, Box<dyn Error>>;
}