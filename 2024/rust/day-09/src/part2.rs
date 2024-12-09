use itertools::Itertools;
use std::iter::repeat_n;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let mut blocks = input[0..input.len() - 1]
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(id, mut chunk)| {
            let mut result = vec![(Some(id), chunk.next().unwrap())];
            if let Some(free_block_size) = chunk.next() {
                result.push((None, free_block_size));
            }
            result
        })
        .collect_vec();

    let mut front = 0;
    while front < blocks.len() {
        let (id, size) = blocks[front];
        if id.is_none() {
            let mut back = blocks.len() - 1;
            while front < back {
                let (back_id, back_size) = blocks[back];
                if back_id.is_some() && back_size == size {
                    blocks.swap(front, back);
                    break;
                } else if back_id.is_some() && back_size < size {
                    blocks[back].0 = None;
                    blocks[front].1 -= back_size;
                    blocks.insert(front, (back_id, back_size));
                    break;
                }
                back -= 1;
            }
        }
        front += 1;
    }
    Ok(blocks
        .iter()
        .flat_map(|(id, size)| repeat_n(*id, *size))
        .map(|b| b.unwrap_or(0))
        .enumerate()
        .map(|(idx, block)| idx * block)
        .sum())
}

fn print_blocks(blocks: &[(Option<usize>, usize)]) {
    println!(
        "{}",
        blocks
            .iter()
            .flat_map(|(id, size)| repeat_n(*id, *size))
            .map(|block| block.map_or(".".to_string(), |b| b.to_string()))
            .join("")
    );
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402\n";
        assert_eq!(2858, process(input)?);
        Ok(())
    }
}
