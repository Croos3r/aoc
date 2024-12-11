#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    Ok(parse_rocks(input)
        .iter()
        .map(|rock| count_for(*rock, 25))
        .sum())
}

fn count_for(rock: u64, steps_remaining: usize) -> usize {
    if steps_remaining == 0 {
        return 1;
    }
    if rock == 0 {
        return count_for(1, steps_remaining - 1);
    }
    let rock_str = rock.to_string();
    if rock_str.len() % 2 == 0 {
        let (first, second) = rock_str.split_at(rock_str.len() / 2);
        return count_for(first.parse().unwrap(), steps_remaining - 1)
            + count_for(second.parse().unwrap(), steps_remaining - 1);
    }
    count_for(rock * 2024, steps_remaining - 1)
}

#[inline]
fn parse_rocks(input: &str) -> Vec<u64> {
    input
        .split_once("\n")
        .unwrap()
        .0
        .split(" ")
        .map(|rock| rock.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() -> miette::Result<()> {
        Ok(())
    }
}
