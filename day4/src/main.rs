use std::{collections::HashMap, fs::read_to_string};

type Passport = HashMap<String, String>;

trait ValidatablePassport {
    fn valid(&self) -> bool;
    fn byr_valid(&self) -> bool;
    fn iyr_valid(&self) -> bool;
    fn eyr_valid(&self) -> bool;
    fn hgt_valid(&self) -> bool;
    fn hcl_valid(&self) -> bool;
    fn ecl_valid(&self) -> bool;
    fn pid_valid(&self) -> bool;
}

impl ValidatablePassport for Passport {
    fn valid(&self) -> bool {
        self.byr_valid()
            && self.iyr_valid()
            && self.eyr_valid()
            && self.hgt_valid()
            && self.hcl_valid()
            && self.ecl_valid()
            && self.pid_valid()
    }

    fn byr_valid(&self) -> bool {
        if let Some(byr) = self.get("byr") {
            if let Ok(byr) = byr.parse::<u16>() {
                if byr >= 1920 && byr <= 2002 {
                    return true;
                }
            }
        }
        false
    }

    fn iyr_valid(&self) -> bool {
        if let Some(iyr) = self.get("iyr") {
            if let Ok(iyr) = iyr.parse::<u16>() {
                if iyr >= 2010 && iyr <= 2020 {
                    return true;
                }
            }
        }
        false
    }

    fn eyr_valid(&self) -> bool {
        if let Some(eyr) = self.get("eyr") {
            if let Ok(eyr) = eyr.parse::<u16>() {
                if eyr >= 2020 && eyr <= 2030 {
                    return true;
                }
            }
        }
        false
    }

    fn hgt_valid(&self) -> bool {
        if let Some(hgt) = self.get("hgt") {
            if hgt.find("cm") != None {
                let hgt = &hgt[0..hgt.len() - 2];
                if let Ok(hgt) = hgt.parse::<u8>() {
                    if hgt >= 150 && hgt <= 193 {
                        return true;
                    }
                }
            } else if hgt.find("in") != None {
                let hgt = &hgt[0..hgt.len() - 2];
                if let Ok(hgt) = hgt.parse::<u8>() {
                    if hgt >= 59 && hgt <= 76 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn hcl_valid(&self) -> bool {
        if let Some(hcl) = self.get("hcl") {
            if hcl.len() == 7 {
                let mut hcl = hcl.chars();
                if hcl.next() == Some('#') {
                    for _ in 0..6 {
                        const HEX: [char; 16] = [
                            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd',
                            'e', 'f',
                        ];
                        if let Some(c) = hcl.next() {
                            if !HEX.contains(&c) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    return true;
                }
            }
        }
        false
    }

    fn ecl_valid(&self) -> bool {
        if let Some(ecl) = self.get("ecl") {
            let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            for color in &colors {
                if color == ecl {
                    return true;
                }
            }
        }
        false
    }

    fn pid_valid(&self) -> bool {
        if let Some(pid) = self.get("pid") {
            if pid.len() == 9 {
                if pid.parse::<u32>().is_ok() {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let passports: Vec<Passport> = read_to_string("input.txt")
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
        .collect();

    let valid_passports = passports.iter().filter(|p| p.valid());
    
    println!("{}", valid_passports.count());
}
