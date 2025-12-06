use crate::parse_input;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let (fresh_id_ranges, available_ids) = parse_input(input);

    Ok(available_ids
        .into_iter()
        .filter(|available_id| {
            fresh_id_ranges
                .iter()
                .any(|(start, end)| start <= available_id && available_id <= end)
        })
        .count())
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
        assert_eq!(3, process(input)?);
        Ok(())
    }
}
