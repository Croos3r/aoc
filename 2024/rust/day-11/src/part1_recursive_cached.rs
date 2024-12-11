use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let mut cache = HashMap::new();
    Ok(parse_rocks(input)
        .iter()
        .map(|&rock| count_for(rock, 25, &mut cache))
        .sum())
}

fn count_for(rock: u64, steps_remaining: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if let Some(cached_count) = cache.get(&(rock, steps_remaining)) {
        return *cached_count;
    }
    if steps_remaining == 0 {
        return 1;
    }
    if rock == 0 {
        let value = count_for(1, steps_remaining - 1, cache);
        cache.insert((rock, steps_remaining), value);
        return value;
    }
    let rock_str = rock.to_string();
    if rock_str.len() % 2 == 0 {
        let (first, second) = rock_str.split_at(rock_str.len() / 2);
        let value = count_for(first.parse().unwrap(), steps_remaining - 1, cache)
            + count_for(second.parse().unwrap(), steps_remaining - 1, cache);
        cache.insert((rock, steps_remaining), value);
        return value;
    }
    let value = count_for(rock * 2024, steps_remaining - 1, cache);
    cache.insert((rock, steps_remaining), value);
    value
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
