use std::{collections::HashMap, ffi::OsStr, fs::{self, DirEntry}, io, path::{Path, PathBuf}};

use serde::Deserialize;
use thiserror::Error;

use crate::{puzzles::{Day, Scenario}, solving::{ResolutionError, ScenarioData}, Puzzle, Scope, Solver};

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

#[derive(Debug, Error)]
pub enum Error {
    #[error("Puzzle input file didn't match the 'day_dd.*' format: {0}")]
    InvalidFileName(String),
    #[error("Failed to parse year from directory name: {0}")]
    InvalidDirectoryName(String),
    #[error("Failed to open data directory")]
    DataDirectory(#[from] io::Error)
}

fn locate_day_manifest(entry: &DirEntry) -> Result<(u32, PathBuf), Error> {
    let filename_os = entry.file_name();
    let filename = filename_os
        .to_str()
        .unwrap();

    let path = entry.path();
    let extension = path
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    let day = filename
        .strip_prefix("day_")
        .and_then(|name| name.strip_suffix(extension))
        .and_then(|name| name.strip_suffix('.'))
        .and_then(|name| name.parse().ok())
        .ok_or_else(|| Error::InvalidFileName(filename.to_owned()))?;

    Ok((day, path))
}

fn locate_year_manifest(entry: &DirEntry) -> Result<Vec<(Day, PathBuf)>, Error> {
    let dirname_os = entry.file_name();
    let dirname = dirname_os
        .to_str()
        .unwrap();

    let year: u32 = dirname
        .parse()
        .map_err(|_| Error::InvalidDirectoryName(dirname.to_owned()))?;

    let paths = fs::read_dir(entry.path())?
        .map(|entry| {
            let (day, path) = locate_day_manifest(&entry?)?;
            Ok((Day { year, day }, path))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(paths)
}

pub fn locate_manifests(location: &Path, scope: Scope) -> Result<HashMap<Day, PathBuf>, Error> {
    let manifests = fs::read_dir(location)?
        .map(|entry| locate_year_manifest(&entry?))
        .collect::<Result<Vec<Vec<(Day, PathBuf)>>, Error>>()?
        .into_iter()
        .flatten()
        .filter(|(day, _)| scope.contains_day(*day))
        .collect();
    
    Ok(manifests)
}