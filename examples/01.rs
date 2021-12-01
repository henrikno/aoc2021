use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    part1("data/01.example");
    part1("data/01.input");
    part2("data/01.example");
    part2("data/01.input");
    Ok(())
}

fn part1(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut prev: i32 = 10000;
    let mut increases = 0;
    for line in reader.lines() {
        let value = line?.parse::<i32>()?;
        if value > prev {
            increases += 1;
        }
        prev = value;
    }
    println!("{}", increases);
    Ok(())
}

fn part2(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut increases = 0;
    let mut ints = Vec::new();
    for line in reader.lines() {
        let value = line?.parse::<i32>()?;
        ints.push(value);
    }

    let mut prev: i32 = 10000;
    for i in 0..ints.len()-2 {
        let a = ints[i+0] + ints[i+1] + ints[i+2];
        if a > prev {
            increases += 1;
        }
        prev = a;
    }

    println!("{}", increases);
    Ok(())
}
