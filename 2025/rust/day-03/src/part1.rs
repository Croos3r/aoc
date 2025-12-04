use crate::{parse_input, FirstMax};
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let banks = parse_input(input)?;

    Ok(banks
        .into_iter()
        .map(|bank| bank.iter().map(|battery| *battery as u32).collect_vec())
        .map(|bank| get_max_output_for_bank(bank))
        .sum())
}

fn get_max_output_for_bank(bank: Vec<u32>) -> u32 {
    let first_max_idx = bank[..bank.len() - 1].iter().position_first_max().unwrap();
    let first_max = bank[first_max_idx];
    let second_max = bank[first_max_idx + 1..].iter().max().unwrap();

    first_max * 10 + second_max
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
