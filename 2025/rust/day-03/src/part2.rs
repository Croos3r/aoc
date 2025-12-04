use crate::{parse_input, FirstMax};
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64> {
    let banks = parse_input(input)?;

    Ok(banks
        .into_iter()
        .map(|bank| bank.iter().map(|battery| *battery as u64).collect_vec())
        .map(|bank| dbg!(get_max_output_for_bank(dbg!(bank))))
        .sum())
}

fn get_max_output_for_bank(bank: Vec<u64>) -> u64 {
    let mut result = 0u64;
    let mut next_idx = 0;

    for i in 0..12 {
        next_idx += (i != 0) as usize;
        next_idx += bank[next_idx..bank.len() - (11 - i)]
            .iter()
            .position_first_max()
            .unwrap();
        result *= 10;
        result += bank[next_idx];
    }

    result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(3_121_910_778_619, process(input)?);
        Ok(())
    }
}
