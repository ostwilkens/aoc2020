use regex::Regex;
use std::{char::ParseCharError, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Password {
    min: u8,
    max: u8,
    char: char,
    phrase: String,
}

impl Password {
    pub fn valid(&self) -> bool {
        let count = self.phrase.chars().filter(|&c| c == self.char).count() as u8;
        return count >= self.min && count <= self.max;
    }
}

#[derive(Debug)]
pub enum ParsePasswordError {
    Regex(regex::Error),
    ParseInt(ParseIntError),
    ParseChar(ParseCharError),
    NoMatch(String),
}

impl From<regex::Error> for ParsePasswordError {
    fn from(err: regex::Error) -> Self {
        ParsePasswordError::Regex(err)
    }
}

impl From<ParseIntError> for ParsePasswordError {
    fn from(err: ParseIntError) -> Self {
        ParsePasswordError::ParseInt(err)
    }
}

impl From<ParseCharError> for ParsePasswordError {
    fn from(err: ParseCharError) -> Self {
        ParsePasswordError::ParseChar(err)
    }
}

impl FromStr for Password {
    type Err = ParsePasswordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = Regex::new(r"(\d+)-(\d+) (\w): (\w+)")?
            .captures(s)
            .ok_or(ParsePasswordError::NoMatch(s.to_string()))?;

        Ok(Password {
            min: cap[1].parse::<u8>()?,
            max: cap[2].parse::<u8>()?,
            char: cap[3].parse::<char>()?,
            phrase: cap[4].to_string(),
        })
    }
}
