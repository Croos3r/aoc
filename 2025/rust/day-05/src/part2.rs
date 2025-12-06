use crate::parse_input;
use itertools::Itertools;
use std::cmp::max;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64> {
    let (mut fresh_id_ranges, _) = parse_input(input);
    fresh_id_ranges.sort();

    let mut result = 0;
    let mut cur = 0;
    for (mut start, end) in fresh_id_ranges {
        if cur >= start {
            start = cur + 1;
        }
        if start <= end {
            result += end - start + 1;
        }
        cur = max(cur, end);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(14, process(input)?);
        Ok(())
    }
}
