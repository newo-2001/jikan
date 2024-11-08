use std::{fmt::{Display, Formatter}, iter};

use serde::Deserialize;

use crate::{DataManifest, DayManifest, PuzzleManifest, Scope};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Puzzle {
    pub year: u32,
    pub day: u32,
    pub part: usize
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub (crate) enum Scenario {
    Puzzle(Puzzle),
    Example {
        puzzle: Puzzle,
        number: usize
    }
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Puzzle(Puzzle { year, day, part }) => {
                write!(f, "{year:04} day {day:02} part {part}")
            },
            Self::Example { puzzle: Puzzle { year, day, part }, number } => {
                write!(f, "{year:04} day {day:02} part {part} (example {number})")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct Day {
    pub year: u32,
    pub day: u32
}

impl TryFrom<String> for Day {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.split_once('-')
            .and_then(|(year, day)| Some(Self {
                year: year.parse().ok()?,
                day: day.parse().ok()?
            })).ok_or("Day has to be in the format: yyyy-dd")
    }
}

fn scenarios_for_puzzle(puzzle: Puzzle, manifest: &PuzzleManifest, include_examples: bool) -> impl Iterator<Item=Scenario> {
    let examples = if include_examples {
        (1..=manifest.examples.len())
            .map(move |number| Scenario::Example { puzzle, number })
            .collect()
    } else { Vec::new() };

    examples.into_iter().chain(iter::once(Scenario::Puzzle(puzzle)))
}

fn scenarios_for_day(day: Day, manifest: &DayManifest, include_examples: bool) -> impl Iterator<Item=Scenario> + '_ {
    manifest.parts
        .iter()
        .enumerate()
        .flat_map(move |(part, manifest)| {
            let puzzle = Puzzle {
                year: day.year,
                day: day.day,
                part: part + 1
            };

            scenarios_for_puzzle(puzzle, manifest, include_examples)
        })
}

impl Scope {
    pub (crate) fn scenarios(self, manifest: &DataManifest, include_examples: bool) -> Vec<Scenario> {
        match self {
            Scope::All => manifest.puzzles
                .iter()
                .flat_map(|(&day, manifest)| scenarios_for_day(day, manifest, include_examples))
                .collect(),
            Scope::Year(year) => manifest.puzzles
                .iter()
                .filter(|(day, _)| day.year == year)
                .flat_map(|(&day, manifest)| scenarios_for_day(day, manifest, include_examples))
                .collect(),
            Scope::Day(day) => manifest.puzzles
                .get(&day)
                .map(|manifest| scenarios_for_day(day, manifest, include_examples).collect())
                .unwrap_or_default(),
            Scope::Puzzle(puzzle) => manifest.puzzles
                .get(&Day { year: puzzle.year, day: puzzle.day })
                .and_then(|day| day.parts.get(puzzle.part))
                .map(|manifest| scenarios_for_puzzle(puzzle, manifest, include_examples).collect())
                .unwrap_or_default()
        }
    }
}