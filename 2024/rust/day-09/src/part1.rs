use itertools::{repeat_n, Itertools};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let mut blocks = input[0..input.len() - 1]
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(id, mut chunk)| {
            let result = repeat_n(Some(id), chunk.next().unwrap());
            if let Some(free_block_size) = chunk.next() {
                result.chain(repeat_n(None, free_block_size)).collect_vec()
            } else {
                result.collect_vec()
            }
        })
        .collect_vec();

    let mut back_idx = blocks.len() - 1;
    for idx in 0..blocks.len() {
        if blocks[idx].is_none() {
            change_back_index(&mut back_idx, &blocks);
            if back_idx <= idx {
                break;
            }
            blocks.swap(idx, back_idx);
        }
    }

    Ok(blocks
        .iter()
        .filter_map(|&b| b)
        .enumerate()
        .map(|(idx, block)| idx * block)
        .sum())
}

fn change_back_index(idx: &mut usize, blocks: &[Option<usize>]) {
    while blocks[*idx].is_none() {
        *idx -= 1;
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402\n";
        assert_eq!(1928, process(input)?);
        let input = "12345\n";
        assert_eq!(60, process(input)?);
        Ok(())
    }
}
