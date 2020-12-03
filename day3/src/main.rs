use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let state = BufReader::new(File::open("input.txt")?)
        .lines()
        .filter_map(|l| l.ok())
        .skip(1)
        .fold((0, 0), |(mut x, mut trees), l| {
            x += 3;

            if l.as_bytes()[x % 31] == 35 {
                trees += 1;
            }

            (x, trees)
        });

    println!("{:?}", state.1);

    Ok(())
}
