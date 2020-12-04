use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), std::io::Error> {
    let lines = BufReader::new(File::open("input.txt")?)
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>();

    let mut total: u64 = 1;
    for (step_x, step_y) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        total *= lines
            .iter()
            .skip(*step_y)
            .step_by(*step_y)
            .fold((0, 0), |(mut x, mut trees), l| {
                x += step_x;

                if l.as_bytes()[x % 31] == 35 {
                    trees += 1;
                }

                (x, trees)
            })
            .1
    }

    println!("{:?}", total);

    Ok(())
}
