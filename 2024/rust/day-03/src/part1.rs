use miette::miette;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, digit1};
use nom::combinator::map_res;
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, preceded, separated_pair};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let (_, operations) =
        many0(many_till(anychar, parse_mul))(input).map_err(|e| miette!(e.to_string()))?;
    Ok(operations.iter().map(|(_, (a, b))| a * b).sum::<u32>())
}

fn parse_mul(input: &str) -> nom::IResult<&str, (u32, u32)> {
    preceded(
        tag("mul"),
        delimited(
            char('('),
            separated_pair(
                map_res(digit1, str::parse::<u32>),
                char(','),
                map_res(digit1, str::parse::<u32>),
            ),
            char(')'),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, process(input)?);
        Ok(())
    }
}
