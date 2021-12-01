use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::error::Error;
use itertools::Itertools;

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
    let ints = file_to_ints::<i32>(reader);
    return Ok(increments(ints.into_iter().tuple_windows().map(|(x, y, z)| x + y + z)));
}

fn increments<I>(ints: I) -> i32
where I: IntoIterator<Item = i32>,
{
    ints.into_iter().tuple_windows().filter(|(a, b)| a < b).count() as i32
}

fn file_to_ints<T: std::str::FromStr>(reader: BufReader<File>) -> impl Iterator<Item=T> {
    reader.lines()
        .flat_map(|x| x.map(|y| y.parse::<T>()))
        .flatten()
}
