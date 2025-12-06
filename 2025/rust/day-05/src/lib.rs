use itertools::Itertools;

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (fresh_ranges, available_ids) = input.split_once("\n\n").unwrap();

    let fresh_ranges = fresh_ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();

            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect_vec();

    let available_ids = available_ids
        .lines()
        .map(|id| id.parse().unwrap())
        .collect_vec();

    (fresh_ranges, available_ids)
}
