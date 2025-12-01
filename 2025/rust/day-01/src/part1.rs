use nom::branch::alt;
use nom::character::complete::{char, line_ending, u16};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::{Finish, Parser};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Rotation {
    left: bool,
    degrees: u16,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.left { "L" } else { "R" }, self.degrees)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let (_, list) = separated_list1(
        line_ending::<&str, nom::error::Error<&str>>,
        map(pair(alt((char('R'), char('L'))), u16), |(dir, deg)| {
            Rotation {
                left: dir == 'L',
                degrees: deg,
            }
        }),
    )
    .parse(input)
    .finish()
    .map_err(|e| miette::miette!("parsing error: {}", e))?;

    let mut cur = 50isize;
    let mut result = 0;

    for rotation in list {
        cur += rotation.degrees as isize * if rotation.left { 1 } else { -1 };
        cur %= 100;
        if cur == 0 {
            result += 1;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(3, process(input)?);
        Ok(())
    }
}
