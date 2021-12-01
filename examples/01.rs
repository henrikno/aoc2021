use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(part1("data/01.example")?, 7);
    assert_eq!(part1("data/01.input")?, 1448);
    assert_eq!(part2("data/01.example")?, 5);
    assert_eq!(part2("data/01.input")?, 1471);
    Ok(())
}

fn part1(filename: &str) -> Result<i32, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(increments(file_to_ints(reader)))
}


fn part2(filename: &str) -> Result<i32, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let ints = file_to_ints(reader);
    return Ok(increments(ints.windows(3).map(|x|x.iter().sum()).collect()));
}

fn increments(ints: Vec<i32>) -> i32 {
    ints.windows(2).filter(|val| val[0] < val[1]).count() as i32
}

fn file_to_ints(reader: BufReader<File>) -> Vec<i32> {
    reader.lines()
        .flat_map(|x| x.map(|y| y.parse::<i32>()))
        .flatten()
        .collect()
}
