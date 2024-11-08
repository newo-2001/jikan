use std::fmt::{Display, Formatter};

use serde::Deserialize;

use crate::{DataManifest, DayManifest, Scope};

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
    pub (crate) fn get_day(self) -> Day {
        Day {
            year: self.year,
            day: self.day
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

fn puzzles_for_day(day: Day, manifest: &DayManifest) -> impl Iterator<Item=Puzzle> {
    (1..=manifest.parts.len()).map(move |part| Puzzle {
        year: day.year,
        day: day.day,
        #[allow(clippy::cast_possible_truncation)]
        part: part as u32
    })
}

impl Scope {
    pub (crate) fn puzzles(self, manifest: &DataManifest) -> Vec<Puzzle> {
        match self {
            Scope::All => manifest.puzzles
                .iter()
                .flat_map(|(&day, manifest)| puzzles_for_day(day, manifest))
                .collect(),
            Scope::Year(year) => manifest.puzzles
                .iter()
                .filter(|(day, _)| day.year == year)
                .flat_map(|(&day, manifest)| puzzles_for_day(day, manifest))
                .collect(),
            Scope::Day(scope) => manifest.puzzles
                .iter()
                .filter(|&(day, _)| *day == scope)
                .flat_map(|(&day, manifest)| puzzles_for_day(day, manifest))
                .collect(),
            Scope::Puzzle(puzzle) => vec![puzzle]
        }
    }
}