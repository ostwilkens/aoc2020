mod password;

use password::Password;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let valid_password_count = BufReader::new(File::open("input.txt")?)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Password>().ok())
        .filter(|pw| pw.valid_part_two())
        .count();

    println!("{}", valid_password_count);

    Ok(())
}
