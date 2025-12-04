use itertools::Itertools;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{Finish, IResult, Parser};

pub mod part1;
pub mod part2;

pub trait FirstMax: Iterator {
    fn position_first_max(self) -> Option<usize>
    where
        Self: Sized + DoubleEndedIterator + ExactSizeIterator,
        Self::Item: Ord + Clone,
    {
        self.enumerate()
            .rev()
            .max_by_key(|(_, item)| item.clone())
            .map(|(index, _)| index)
    }

    fn first_max(self) -> Option<Self::Item>
    where
        Self: Sized + DoubleEndedIterator,
        Self::Item: Ord,
    {
        self.rev().max()
    }
}

impl<T> FirstMax for T where T: Iterator + ?Sized {}

fn parse_input(input: &str) -> miette::Result<Vec<Vec<u8>>> {
    let (_, banks) = separated_list1(line_ending, parse_bank)
        .parse(input)
        .finish()
        .map_err(|err| miette::miette!("parsing error: {err}"))?;
    Ok(banks)
}

fn parse_bank(input: &str) -> IResult<&str, Vec<u8>> {
    map(digit1, |digits: &str| {
        digits
            .chars()
            .map(|digit| digit.to_digit(10).map(|n| n as u8).unwrap())
            .collect_vec()
    })
    .parse(input)
}
