use std::collections::HashMap;

type Passport = HashMap<String, String>;

fn main() {
    // fugly, no time

    let passports = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|l| {
            l.replace("\n", " ")
                .trim_end()
                .split(" ")
                .map(|pair| {
                    let key = &pair[0..3];
                    let value = &pair[4..pair.len()];

                    (key.to_string(), value.to_string())
                })
                .collect::<Passport>()
        })
        .collect::<Vec<Passport>>();

    let mut valid_passport_count = 0;
    for passport in passports {
        let mut valid_fields = 0;

        if let Some(byr) = passport.get("byr") {
            if let Ok(byr) = byr.parse::<u16>() {
                if byr >= 1920 && byr <= 2002 {
                    valid_fields += 1;
                }
            }
        }

        if let Some(iyr) = passport.get("iyr") {
            if let Ok(iyr) = iyr.parse::<u16>() {
                if iyr >= 2010 && iyr <= 2020 {
                    valid_fields += 1;
                }
            }
        }

        if let Some(eyr) = passport.get("eyr") {
            if let Ok(eyr) = eyr.parse::<u16>() {
                if eyr >= 2020 && eyr <= 2030 {
                    valid_fields += 1;
                }
            }
        }

        if let Some(hgt) = passport.get("hgt") {
            if hgt.find("cm") != None {
                let hgt = &hgt[0..hgt.len() - 2];
                if let Ok(hgt) = hgt.parse::<u8>() {
                    if hgt >= 150 && hgt <= 193 {
                        valid_fields += 1;
                    }
                }
            } else if hgt.find("in") != None {
                let hgt = &hgt[0..hgt.len() - 2];
                if let Ok(hgt) = hgt.parse::<u8>() {
                    if hgt >= 59 && hgt <= 76 {
                        valid_fields += 1;
                    }
                }
            }
        }

        if let Some(hcl) = passport.get("hcl") {
            if hcl.len() == 7 {
                let mut hcl = hcl.chars();
                if hcl.next() == Some('#') {
                    let mut valid_characters = 0;
                    for _ in 0..6 {
                        let hex = [
                            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd',
                            'f',
                        ];
                        if let Some(c) = hcl.next() {
                            if hex.contains(&c) {
                                valid_characters += 1;
                            }
                        }
                    }

                    if valid_characters == 6 {
                        valid_fields += 1;
                    }
                }
            }
        }

        if let Some(ecl) = passport.get("ecl") {
            let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            for color in &colors {
                if color == ecl {
                    valid_fields += 1;
                }
            }
        }

        if let Some(pid) = passport.get("pid") {
            if pid.len() == 9 {
                if pid.parse::<u32>().is_ok() {
                    valid_fields += 1;
                }
            }
        }

        if valid_fields >= 7 {
            valid_passport_count += 1;
        }
    }

    println!("{}", valid_passport_count);
}
