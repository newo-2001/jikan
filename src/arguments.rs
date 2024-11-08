#![allow(clippy::missing_errors_doc)]

use std::fmt::Display;

use clap::Parser;

use crate::puzzles::{Day, Puzzle};

#[derive(Parser, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[command(version, about, long_about = None)]
pub struct ExecutionOptions {
    #[arg(
        long,
        short = 'v',
        help = "Verify solutions"
    )]
    pub verify: bool,
    
    #[arg(
        long,
        short = 'x',
        help = "Execute examples"
    )]
    pub examples: bool,

    #[arg(
        long,
        short = 's',
        value_parser = Scope::parse,
        default_value_t = Scope::All,
        help = "The scope for which puzzles should execute",
        long_help = "The scope for which puzzles should execute in the format: [all|yyyy[-dd[-p]]] (default: all)"
    )]
    pub scope: Scope
}

impl ExecutionOptions {
    #[must_use] pub fn from_args() -> Self {
        ExecutionOptions::parse()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Scope {
    All,
    Year(u32),
    Day(Day),
    Puzzle(Puzzle)
}

impl Scope {
    fn parse(value: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = value.split('-').collect();
        let parse = || {
            Some(match parts.as_slice() {
                [year, day, part] => Scope::Puzzle(Puzzle {
                    year: year.parse().ok()?,
                    day: day.parse().ok()?,
                    part: part.parse().ok()?,
                }),
                [year, day] => Scope::Day(Day {
                    year: year.parse().ok()?,
                    day: day.parse().ok()?
                }),
                [] | ["all"] => Scope::All,
                [year] => Scope::Year(year.parse().ok()?),
                _ => return None
            })
        };

        parse().ok_or("Scope should be in the format: [all|yyyy[-dd[-p]]]] ")
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Scope::All => write!(f, "all"),
            Scope::Year(year) => write!(f, "{year:0>4}"),
            Scope::Day(Day { year, day }) => write!(f, "{year:0>4}-{day:0>2}"),
            Scope::Puzzle(Puzzle { year, day, part }) => write!(f, "{year:0>4}-{day:0>2}-{part}")
        }
    }
}