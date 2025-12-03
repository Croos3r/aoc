use crate::{get_invalid_ids_for_range, parse_id_ranges};
use itertools::{Chunk, Itertools};
use nom::Finish;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64> {
    let (_, id_ranges) = parse_id_ranges(input)
        .finish()
        .map_err(|e| miette::miette!("parsing error: {}", e))?;
    Ok(id_ranges
        .into_iter()
        .flat_map(|(a, b)| dbg!(get_invalid_ids_for_range(dbg!(a), dbg!(b), is_invalid_id)))
        .sum())
}

fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();

    for size in (1..=id_str.len() / 2)
        .rev()
        .filter(|size| id_str.len() % size == 0)
    {
        let chunks: Vec<String> = id_str
            .as_str()
            .chars()
            .chunks(size)
            .into_iter()
            .map(Chunk::collect)
            .collect_vec();
        if chunks.into_iter().all_equal() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(4_174_379_265, process(input)?);
        Ok(())
    }
}
