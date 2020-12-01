use std::{
    fs::{read_to_string, File},
    io::{self, BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    for line in read_to_string("input.txt")?.split('\n') {
        println!("{}", line);
    }

    Ok(())
}
