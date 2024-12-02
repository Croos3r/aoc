use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let result = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|report| {
            if is_report_safe(report) {
                return true;
            }

            if (0..report.len()).any(|i| {
                let mut report_copy = report.clone();
                report_copy.remove(i);
                is_report_safe(&report_copy)
            }) {
                return true;
            }
            false
        })
        .collect::<Vec<_>>();
    Ok(result.len())
}

fn is_report_safe(report: &[u32]) -> bool {
    (report.is_sorted() || report.is_sorted_by(|a, b| b <= a))
        && !report
            .windows(2)
            .any(|w| !(1..=3).contains(&w[0].abs_diff(w[1])))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        //				  - 1 2 2 1    1 5 1 1  - 2 1 4 3    2[1]2 1  - 2 2 0 3    2 3 1 2
        assert_eq!(4, process(input)?);
        Ok(())
    }
}
