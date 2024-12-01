#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
            let a = numbers.next().unwrap();
            let b = numbers.next().unwrap();
            (a, b)
        })
        .unzip();
    left.sort();
    right.sort();
    Ok(left.iter().zip(right).map(|(&a, b)| a.abs_diff(b)).sum())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(11, process(input)?);
        Ok(())
    }
}
