use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("day5/input.txt").unwrap();

    let mut lines = BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>();

    lines.sort();

    let mut prev_id = 0;
    let mut max_id = 0;
    for line in lines {
        let row = line[0..7]
            .replace("F", "\x00")
            .replace("B", "\x01")
            .bytes()
            .rev()
            .enumerate()
            .fold(0, |x, (i, b)| x | (b << i));

        let col = line[7..10]
            .replace("L", "\x00")
            .replace("R", "\x01")
            .bytes()
            .rev()
            .enumerate()
            .fold(0, |x, (i, b)| x | (b << i));

        let id = row as u16 * 8 + col as u16;

        if id + 14 == prev_id {
            println!("part 2: {:?}", prev_id - 15);
        }

        if id == prev_id + 2 {
            println!("part 2: {:?}", prev_id + 1);
        }

        prev_id = id;
        max_id = max_id.max(id);
    }

    println!("part 1: {:?}", max_id);
}
