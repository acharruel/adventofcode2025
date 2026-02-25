use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
    time,
};

use anyhow::Result;
use nanospinner::Spinner;
use strum::{EnumIter, FromRepr, IntoEnumIterator};
use strum_macros::Display;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod utils;

pub trait AocRun {
    fn run(&self, name: &str) -> Result<()> {
        println!("{name}:");
        let spinner = Spinner::new(" Running part 1").start();
        let start = time::Instant::now();
        match self.run1() {
            Ok(res) => spinner.success_with(format!(
                " Part 1: {} in {:.2} us",
                res,
                start.elapsed().as_micros()
            )),
            Err(_) => spinner.fail(),
        }
        let spinner = Spinner::new(" Running part 2").start();
        match self.run2() {
            Ok(res) => spinner.success_with(format!(
                " Part 2: {} in {:.2} us",
                res,
                start.elapsed().as_micros()
            )),
            Err(e) => spinner.fail_with(format!(" {}", e)),
        }
        Ok(())
    }
    fn run1(&self) -> Result<i64>;
    fn run2(&self) -> Result<i64>;
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
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
}

impl From<Day> for Box<dyn AocRun> {
    fn from(value: Day) -> Self {
        match value {
            Day::Day00 => Box::new(Day00),
            Day::Day01 => Box::new(day01::Day01),
            Day::Day02 => Box::new(day02::Day02),
            Day::Day03 => Box::new(day03::Day03),
            Day::Day04 => Box::new(day04::Day04),
            Day::Day05 => Box::new(day05::Day05),
            Day::Day06 => Box::new(day06::Day06),
            Day::Day07 => Box::new(day07::Day07),
            Day::Day08 => Box::new(day08::Day08),
            Day::Day09 => Box::new(day09::Day09),
            Day::Day10 => Box::new(day10::Day10),
            Day::Day11 => Box::new(day11::Day11),
            Day::Day12 => Box::new(day12::Day12),
        }
    }
}

struct Day00;

impl AocRun for Day00 {
    fn run(&self, _name: &str) -> Result<()> {
        for day in Day::iter() {
            match day {
                Day::Day00 => (),
                _ => {
                    let name = &day.to_string();
                    let module: Box<dyn AocRun> = day.into();
                    module.run(name)?;
                }
            }
        }
        Ok(())
    }
    fn run1(&self) -> Result<i64> {
        Ok(0)
    }
    fn run2(&self) -> Result<i64> {
        Ok(0)
    }
}

pub fn run(day: u8) -> Result<()> {
    if let Some(day) = Day::from_repr(day) {
        let name = &day.to_string();
        println!("Running {}", name);
        let module: Box<dyn AocRun> = day.into();
        module.run(name)?;
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
