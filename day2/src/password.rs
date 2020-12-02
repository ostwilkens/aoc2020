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
    pub fn valid_part_one(&self) -> bool {
        let count = self.phrase.chars().filter(|&c| c == self.char).count() as u8;
        return count >= self.min && count <= self.max;
    }

    pub fn valid_part_two(&self) -> bool {
        let chars = self.phrase.chars().collect::<Vec<char>>();
        let a = chars.get(self.min as usize - 1);
        let b = chars.get(self.max as usize - 1);

        match (a, b) {
            (Some(&a), None) if a == self.char => true,
            (Some(&a), Some(&b)) if a == self.char && b != self.char => true,
            (Some(&a), Some(&b)) if b == self.char && a != self.char => true,
            _ => false,
        }
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
