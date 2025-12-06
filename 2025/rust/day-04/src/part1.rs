use crate::{find_retrievable_rolls_in_map, parse_input};
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let map = parse_input(input);

    Ok(find_retrievable_rolls_in_map(&map).len())
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
@.@.@@@.@.";
        assert_eq!(13, process(input)?);
        Ok(())
    }
}
