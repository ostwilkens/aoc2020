#![feature(iterator_fold_self)]

use std::{collections::HashSet, fs::read_to_string};

fn main() {
    println!(
        "part 1: {:?}",
        read_to_string("day6/input.txt")
            .unwrap()
            .split("\n\n")
            .map(|g| g.replace("\n", "").chars().collect::<HashSet<char>>().len())
            .sum::<usize>()
    );

    println!(
        "part 2: {:?}",
        read_to_string("day6/input.txt")
            .unwrap()
            .split("\n\n")
            .map(|g| g
                .split("\n")
                .map(|p| p.chars().collect::<HashSet<char>>())
                .fold_first(|acc, x| {
                    x.intersection(&acc).map(|&c| c).collect::<HashSet<char>>()
                })
                .unwrap()
                .len())
            .sum::<usize>()
    );
}
