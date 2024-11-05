#![allow(clippy::missing_errors_doc)]

use std::env;

use nom::{combinator::{eof, all_consuming, value}, character::complete::{u32, char}, sequence::{separated_pair, tuple, preceded}, Parser, branch::alt};
use thiserror::Error;

use crate::{ExecutionOptions, Puzzle};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Action {
    Run,
    Verify
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Scope {
    All,
    Year(u32),
    Day(u32, u32),
    Puzzle(Puzzle)
}

impl Scope {
    pub (crate) fn parse(input: &str) -> Result<Self, nom::Err<nom::error::Error<&str>>> {
        Ok(all_consuming(alt((
            value(Self::All, eof),
            tuple((
                u32,
                preceded(char('-'), u32),
                preceded(char('-'), u32)
            )).map(|(year, day, part)| {
                Self::Puzzle(Puzzle { year, day, part })
            }),
            separated_pair(u32, char('-'), u32)
                .map(|(year, day)| Self::Day(year, day)),
            u32.map(Self::Year)
        ))).parse(input)?.1)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Parsing(#[from] nom::Err<nom::error::Error<String>>),
    #[error("Encountered unexpected position argument: `{0}`")]
    UnexpectedArgument(String)
}

// TODO: clean this up, possibly using clap
impl ExecutionOptions {
    pub fn from_args() -> Result<Self, Error> {
        let arguments = env::args()
            .skip(1)
            .collect::<Vec<_>>();

        let args = arguments.iter()
            .map(String::as_str)
            .collect::<Vec<_>>();

        match args.as_slice() {
            ["verify", scope] => {
                let scope = Scope::parse(scope)
                    .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;

                Ok(Self { scope, action: Action::Verify })
            },
            ["verify"] | [] => Ok(Self { scope: Scope::All, action: Action::Verify }),
            [scope] => {
                let scope = Scope::parse(scope)
                    .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;

                Ok(Self { scope, action: Action::Run })
            },
            ["verify", _, arg] | [_, arg, ..] => {
                Err(Error::UnexpectedArgument((*arg).to_owned()))
            }
        }
    }
}
