#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let result = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| report.is_sorted() || report.is_sorted_by(|a, b| b <= a))
        .filter(|report| {
            !report
                .windows(2)
                .any(|w| !(1..=3).contains(&w[0].abs_diff(w[1])))
        })
        .count();
    Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        assert_eq!(2, process(input)?);
        Ok(())
    }
}
