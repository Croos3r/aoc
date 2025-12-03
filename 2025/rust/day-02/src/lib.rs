use itertools::Itertools;
use nom::character::complete::{char, u64};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

pub mod part1;
pub mod part2;

fn get_invalid_ids_for_range<F>(start: u64, end: u64, is_invalid_id: F) -> Vec<u64>
where
    F: Fn(u64) -> bool,
{
    assert!(start < end);
    (start..=end)
        .into_iter()
        .filter(|&i| is_invalid_id(i))
        .collect_vec()
}

fn parse_id_ranges(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(char(','), parse_id_range).parse(input)
}

fn parse_id_range(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(u64, char('-'), u64).parse(input)
}
