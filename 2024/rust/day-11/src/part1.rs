use itertools::Itertools;
use std::ops::Mul;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let mut rocks = parse_rocks(input);
    for _ in 0..25 {
        rocks = rocks
            .into_iter()
            .flat_map(|rock| {
                if rock == "0" {
                    vec!["1".to_string()]
                } else if rock.len() % 2 == 0 {
                    let (first, second) = rock.split_at(rock.len() / 2);
                    vec![
                        first.to_string().parse::<u64>().unwrap().to_string(),
                        second.to_string().parse::<u64>().unwrap().to_string(),
                    ]
                } else {
                    vec![rock.parse::<u64>().unwrap().mul(2024).to_string()]
                }
            })
            .collect_vec();
    }
    Ok(rocks.len())
}

#[inline]
fn parse_rocks(input: &str) -> Vec<String> {
    input
        .split_once("\n")
        .unwrap()
        .0
        .split(" ")
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17\n";
        assert_eq!(55312, process(input)?);
        Ok(())
    }
}
