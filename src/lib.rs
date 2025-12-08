use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
    time::SystemTime,
};

use anyhow::Result;
use strum::{EnumIter, FromRepr};
use strum_macros::Display;
use tracing::info;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use crate::{
    day01::Day01, day02::Day02, day03::Day03, day04::Day04, day05::Day05, day06::Day06,
    day07::Day07,
};

pub trait DDay {
    fn run(&self) -> Result<()>;
}

#[derive(Debug, Display, EnumIter, FromRepr)]
#[repr(u8)]
pub enum Day {
    #[strum(to_string = "all days")]
    Day00,
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
}

impl DDay for Day {
    fn run(&self) -> Result<()> {
        info!("Running {}:", self);
        let start = SystemTime::now();
        match self {
            Day::Day00 => {
                let all: Vec<&dyn DDay> =
                    vec![&Day01, &Day02, &Day03, &Day04, &Day05, &Day06, &Day07];
                all.iter().for_each(|day| {
                    day.run().expect("Failed to run all days");
                    println!();
                });
            }
            Day::Day01 => Day01.run()?,
            Day::Day02 => Day02.run()?,
            Day::Day03 => Day03.run()?,
            Day::Day04 => Day04.run()?,
            Day::Day05 => Day05.run()?,
            Day::Day06 => Day06.run()?,
            Day::Day07 => Day07.run()?,
        }
        info!("took {} us", start.elapsed().unwrap().as_micros());
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
        .map(|l| l.expect("Failed to parse line"))
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
