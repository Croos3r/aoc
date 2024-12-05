use itertools::Itertools;
use miette::miette;
use nom::character::complete;
use nom::character::complete::{char, line_ending};
use nom::error::ParseError;
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;
use std::cmp::Ordering;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let (input, rules) = parse_rules(input).map_err(|e| miette!(e.to_string()))?;
    let (_, updates) = parse_updates(input).map_err(|e| miette!(e.to_string()))?;
    let rules_map: HashMap<_, _> =
        rules
            .clone()
            .into_iter()
            .fold(HashMap::<u32, Vec<u32>>::new(), |mut map, rule| {
                map.entry(rule.0).or_default().push(rule.1);
                map
            });

    Ok(updates
        .iter()
        .filter(|update| idk(update, &rules, false))
        .map(|update| order(update, &rules_map))
        .filter_map(|update| update.get(update.len() / 2).cloned())
        .sum::<u32>())
}

fn order<'a>(update: &'a [u32], rules_map: &HashMap<u32, Vec<u32>>) -> Vec<&'a u32> {
    update
        .iter()
        .sorted_by(|a, b| {
            rules_map
                .get(a)
                .map(|vec| {
                    if vec.contains(b) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                })
                .unwrap_or(Ordering::Less)
        })
        .collect()
}

#[inline]
fn idk(update: &[u32], rules: &[(u32, u32)], ordered: bool) -> bool {
    for rule in rules {
        let Some(left) = update.iter().position(|&x| x == rule.0) else {
            continue;
        };
        let Some(right) = update.iter().position(|&x| x == rule.1) else {
            continue;
        };
        if left > right {
            return !ordered;
        }
    }
    ordered
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many1(terminated(
        separated_pair(complete::u32, char('|'), complete::u32),
        line_ending,
    ))(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many1(preceded(
        line_ending,
        separated_list1(char(','), complete::u32),
    ))(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(123, process(input)?);
        Ok(())
    }
}
