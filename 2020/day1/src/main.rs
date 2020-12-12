use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_file_content() -> io::Result<Vec<i32>> {
    let file = File::open("./src/report.csv")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut expenses = Vec::new();

    while reader.read_line(&mut line)? > 0 {
        // remove newline
        line.pop();
        expenses.push(line.parse::<i32>().unwrap());
        line = String::new();
    }

    Ok(expenses)
}

fn get_counterpart(set: &HashSet::<i32>, number: &i32, sum: &i32) -> Option<i32> {
    let twin = sum - number;
    match set.contains(&twin) {
        true => Some(twin),
        false => None
    }
}

fn main() {
    let expenses = get_file_content().expect("Failed to read report.csv");
    let mut memory = HashSet::<i32>::new();

    /* Part 1 */

    for expense in expenses.iter() {
        if let Some(counter) = get_counterpart(&memory, expense, &2020) {
            print!("Part 1: Expenses: {}, {}. Product: {}\n", expense, counter, expense * counter);
            break;
        }

        memory.insert(*expense);
    }

    /* Part 2 */

    memory.clear();

    for expense in expenses.iter() {

        let twin = 2020 - expense;
        for expense2 in expenses.iter() {
            if let Some(counter) = get_counterpart(&memory, expense2, &twin) {
                print!("Part 2: Expenses: {}, {}, {}. Product: {}\n", expense, expense2, counter, expense * expense2 * counter);
                return;
            }
        }

        memory.insert(*expense);
    }
}
