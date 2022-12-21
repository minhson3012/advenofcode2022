use std::{io::{BufRead, BufReader}, fs::File, path::Path};

mod day1;
fn main() {
    let path = Path::new("./src/day1/input.txt".trim());
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    day1::day_1(lines);
}