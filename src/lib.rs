use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

use anyhow::Result;
use strum::{EnumIter, FromRepr};
use tracing::info;

use crate::{day01::Day01, day02::Day02, day03::Day03};

mod day01;
mod day02;
mod day03;

pub trait DDay {
    fn run(&self) -> Result<()>;
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}

impl Display for dyn DDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{:?}", self.fmt(f));
        Ok(())
    }
}

#[derive(Debug, EnumIter, FromRepr)]
#[repr(u8)]
pub enum Day {
    Day00,
    Day01,
    Day02,
    Day03,
}

impl DDay for Day {
    fn run(&self) -> Result<()> {
        match self {
            Day::Day00 => {
                let all: Vec<&dyn DDay> = vec![&Day01, &Day02];
                all.iter().for_each(|day| {
                    info!("Running {}", day);
                    day.run().expect("Failed to run all days");
                });
            }
            Day::Day01 => Day01.run()?,
            Day::Day02 => Day02.run()?,
            Day::Day03 => Day03.run()?,
        }
        Ok(())
    }
}

pub fn run(day: u8) -> Result<()> {
    if let Some(day) = Day::from_repr(day) {
        day.run()?;
    };
    Ok(())
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    Ok(read_lines(filename)?
        .map(|l| l.expect("Could not parse line"))
        .collect())
}

pub fn single_line_from_file(filename: impl AsRef<Path>) -> Result<String> {
    Ok(read_lines(filename)?.next().unwrap()?)
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
