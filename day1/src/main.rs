use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("day1/input.txt").unwrap();

    let data: HashSet<u32> = BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<u32>().ok())
        .collect();

    for x in &data {
        let y = 2020 - x;
        if data.contains(&y) {
            println!("part 1: {}", x * y);
            break;
        }
    }

    for x in &data {
        for y in &data {
            for z in &data {
                if x + y + z == 2020 {
                    println!("part 2: {}", x * y * z);
                    return;
                }
            }
        }
    }
}
