use glam::UVec2;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let map = parse_map(input);
    let map_size = map.len();
    let trailheads = find_trailheads(&map);

    Ok(trailheads
        .par_iter()
        .map(|trailhead| find_paths_for_trailhead(&map, map_size, *trailhead))
        .sum())
}

fn find_paths_for_trailhead(map: &[Vec<u8>], map_size: usize, trailhead: UVec2) -> usize {
    let mut queue = VecDeque::from([(trailhead, 1u8)]);
    let mut reachable_nines = vec![];

    while let Some((current, searched_height)) = queue.pop_front() {
        let x = current.x as usize;
        let y = current.y as usize;

        if map[x][y] == 9 {
            reachable_nines.push(current);
            continue;
        }

        if x > 0 && map[x - 1][y] == searched_height {
            let up = UVec2::new(x as u32 - 1, y as u32);
            queue.push_back((up, searched_height + 1));
        }

        if x < map_size - 1 && map[x + 1][y] == searched_height {
            let down = UVec2::new(x as u32 + 1, y as u32);
            queue.push_back((down, searched_height + 1));
        }

        if y > 0 && map[x][y - 1] == searched_height {
            let left = UVec2::new(x as u32, y as u32 - 1);
            queue.push_back((left, searched_height + 1));
        }

        if y < map_size - 1 && map[x][y + 1] == searched_height {
            let right = UVec2::new(x as u32, y as u32 + 1);
            queue.push_back((right, searched_height + 1));
        }
    }
    reachable_nines.iter().unique().count()
}

fn find_trailheads(map: &[Vec<u8>]) -> Vec<UVec2> {
    map.iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(y, &c)| (c == 0).then_some(UVec2::new(x as u32, y as u32)))
        })
        .collect_vec()
}

fn parse_map(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|c| c as u8).unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "9990999
9991999
9992999
6543456
7111117
8111118
9111119";
        assert_eq!(2, process(input)?);
        let input = "9990999
9991998
9992997
6543456
7651987
8761111
9871111";
        assert_eq!(4, process(input)?);
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(36, process(input)?);
        Ok(())
    }
}
