use crate::{find_retrievable_rolls_in_map, parse_input, Roll};
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let map = parse_input(input);
    let mut result: Vec<Roll> = vec![];

    loop {
        let filtered_map = map
            .clone()
            .into_iter()
            .filter(|roll| !result.contains(roll))
            .collect_vec();
        let retrievable = find_retrievable_rolls_in_map(&filtered_map);
        if retrievable.is_empty() {
            break;
        }

        for roll in retrievable {
            result.push(roll.clone());
        }
    }

    Ok(result.len())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@";
        assert_eq!(43, process(input)?);
        Ok(())
    }
}
