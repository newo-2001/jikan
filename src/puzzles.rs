use std::{fmt::{Display, Formatter}, iter};

use serde::Deserialize;

use crate::{PuzzleManifest, Scope};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Puzzle {
    pub year: u32,
    pub day: u32,
    pub part: usize
}

impl From<Puzzle> for Day {
    fn from(value: Puzzle) -> Self {
        Day { year: value.year, day: value.day }
    }
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
                write!(f, "{year:04} day {day:02} part {part} ex. {number}")
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

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.day)
    }
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

pub (crate) fn scenarios_for_puzzle(puzzle: Puzzle, manifest: &PuzzleManifest, include_examples: bool) -> impl Iterator<Item=Scenario> {
    let examples = if include_examples {
        (1..=manifest.examples.len())
            .map(move |number| Scenario::Example { puzzle, number })
            .collect()
    } else { Vec::new() };

    examples.into_iter().chain(iter::once(Scenario::Puzzle(puzzle)))
}

impl Scope {
    pub (crate) fn contains_day(self, day: Day) -> bool {
        match self {
            Self::All => true,
            Self::Year(year) => day.year == year,
            Self::Day(day_scope) => day_scope == day,
            Self::Puzzle(Puzzle { day: scope_day, year, .. }) => day.day == scope_day && year == day.year,
            Self::Example { puzzle: Puzzle { day: scope_day, year, .. }, .. } => day.day == scope_day && year == day.year,
        }
    }

    pub (crate) fn contains_puzzle(self, puzzle: Puzzle) -> bool {
        match self {
            Self::All => true,
            Self::Year(year) => year == puzzle.year,
            Self::Day(day) => puzzle.day == day.day && puzzle.year == day.year,
            Self::Puzzle(puzzle_scope) => puzzle_scope == puzzle,
            Self::Example { puzzle: puzzle_scope, .. } => puzzle_scope == puzzle
        }
    }
}