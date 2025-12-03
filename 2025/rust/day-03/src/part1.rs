use itertools::Itertools;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{Finish, IResult, Parser};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let banks = parse_input(input)?;

    Ok(banks.into_iter().map(get_max_output_for_bank).sum())
}

fn get_max_output_for_bank(bank: Vec<u8>) -> u32 {
    let first_max_idx = bank[..bank.len() - 1].iter().position_max().unwrap();
    let first_max = bank[first_max_idx];
    let second_max = &bank[first_max_idx + 1..].iter().max().unwrap();

    format!("{first_max}{second_max}").parse::<u32>().unwrap()
}

fn parse_input(input: &str) -> miette::Result<Vec<Vec<u8>>> {
    let (_, banks) = separated_list1(line_ending, parse_bank)
        .parse(input)
        .finish()
        .map_err(|err| miette::miette!("parsing error: {}", err.to_string()))?;
    Ok(banks)
}

fn parse_bank(input: &str) -> IResult<&str, Vec<u8>> {
    map(digit1, |digits: &str| {
        digits
            .chars()
            .map(|digit| digit.to_digit(10).unwrap() as u8)
            .collect_vec()
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(357, process(input)?);
        Ok(())
    }
}
