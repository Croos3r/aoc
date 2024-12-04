use miette::miette;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, digit1};
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, preceded, separated_pair};

#[derive(Debug)]
enum Operation {
    Mul((u32, u32)),
    Do,
    Dont,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let (_, operations) =
        parse_many_cleaned_operation(input).map_err(|e| miette!(e.to_string()))?;
    Ok(operations
        .iter()
        .scan(true, |enabled, op| match op {
            Operation::Do => {
                *enabled = true;
                Some(None)
            }
            Operation::Dont => {
                *enabled = false;
                Some(None)
            }
            Operation::Mul((a, b)) => Some((*enabled).then_some(a * b)),
        })
        .flatten()
        .sum())
}

fn parse_many_cleaned_operation(input: &str) -> nom::IResult<&str, Vec<Operation>> {
    map_res(
        many0(many_till(
            anychar,
            alt((
                map_res(parse_mul, |mul| Ok::<_, Error<&str>>(Operation::Mul(mul))),
                map_res(tag("do()"), |_| Ok::<_, Error<&str>>(Operation::Do)),
                map_res(tag("don't()"), |_| Ok::<_, Error<&str>>(Operation::Dont)),
            )),
        )),
        |result| {
            Ok::<_, Error<&str>>(
                result
                    .into_iter()
                    .map(|(_, operations)| operations)
                    .collect::<Vec<_>>(),
            )
        },
    )(input)
}

fn parse_mul(input: &str) -> nom::IResult<&str, (u32, u32)> {
    map_res(
        preceded(
            tag("mul"),
            delimited(
                char('('),
                separated_pair(digit1, char(','), digit1),
                char(')'),
            ),
        ),
        |(a, b): (&str, &str)| {
            Ok::<_, Error<&str>>((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        },
    )(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, process(input)?);
        Ok(())
    }
}
