use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

use anyhow::Result;
use strum::FromRepr;

mod day01;
mod day02;

#[derive(Debug, FromRepr, PartialEq)]
#[repr(u8)]
pub enum Day {
    Day00,
    Day01,
    Day02,
}

pub fn run(day: u8) -> Result<()> {
    match Day::from_repr(day) {
        Some(Day::Day00) => (),
        Some(Day::Day01) => day01::run()?,
        Some(Day::Day02) => day02::run()?,
        None => todo!(),
    }

    Ok(())
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    Ok(read_lines(filename)?
        .map(|l| l.expect("Could not parse line"))
        .collect())
}

pub fn single_line_from_file(filename: impl AsRef<Path>) -> Result<String> {
    Ok(read_lines(filename)?.nth(0).unwrap()?)
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
