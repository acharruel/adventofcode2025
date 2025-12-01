use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use anyhow::Result;
use strum::FromRepr;

mod day01;

#[derive(Debug, FromRepr, PartialEq)]
#[repr(u8)]
pub enum Day {
    Day00,
    Day01,
}

pub fn run(day: u8) -> Result<()> {
    match Day::from_repr(day) {
        Some(Day::Day00) => (),
        Some(Day::Day01) => day01::run(),
        None => todo!(),
    }

    Ok(())
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
