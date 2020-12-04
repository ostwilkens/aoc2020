fn main() {
    let keys = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    let result = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|l| keys.iter().all(|k| l.contains(k)))
        .filter(|&x| x)
        .count();

    println!("{}", result);
}
