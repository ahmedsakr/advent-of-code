use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct PasswordEntry {
    password: String,
    letter: String,
    minimum_letter: u32,
    maximum_letter: u32,
}

impl PasswordEntry {

    fn new(line: &String) -> Self {

        let parts: Vec<&str> = line.split(":").collect();
        let policy: Vec<&str> = parts[0].split(" ").collect();
        let policy_range: Vec<&str> = policy[0].split("-").collect();
        let password = parts[1].replace(" ", "");

        PasswordEntry {
            password: password.to_string(),
            minimum_letter: policy_range[0].parse::<u32>().unwrap(),
            maximum_letter: policy_range[1].parse::<u32>().unwrap(),
            letter: policy[1].to_string(),
        }
    }

    fn part1_valid(&self) -> bool {
        let letter_count = self.password.matches(self.letter.chars().nth(0).unwrap()).count() as u32;
        letter_count >= self.minimum_letter && letter_count <= self.maximum_letter
    }

    fn part2_valid(&self) -> bool {
        (self.password.chars().nth((self.minimum_letter - 1) as usize).unwrap() == self.letter.chars().nth(0).unwrap())
        ^ (self.password.chars().nth((self.maximum_letter - 1) as usize).unwrap() == self.letter.chars().nth(0).unwrap())
    }
}

fn get_file_content() -> io::Result<Vec<String>> {
    let file = File::open("./src/list.csv")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut expenses = Vec::new();

    while reader.read_line(&mut line)? > 0 {
        // remove newline
        line.pop();
        expenses.push(line);
        line = String::new();
    }

    Ok(expenses)
}

fn main() {
    let list = get_file_content().expect("Failed to read list.csv");

    let mut part1_valid = 0;
    let mut part2_valid = 0;
    for entry in list.iter() {
        if PasswordEntry::new(entry).part1_valid() {
            part1_valid += 1;
        }
        if PasswordEntry::new(entry).part2_valid() {
            part2_valid += 1;
        }
    }

    print!("Part 1: Total valid: {}\n", part1_valid);
    print!("Part 2: Total valid: {}\n", part2_valid);

}
