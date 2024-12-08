use glam::IVec2;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let map_size = input.lines().next().unwrap().len();
    let antennas = parse_map(input);
    let antenna_pairs = get_antenna_pairs(&antennas);

    let mut antinodes = 0usize;

    for x in 0..map_size {
        for y in 0..map_size {
            let pos = IVec2::new(x as i32, y as i32);
            for (a, b) in &antenna_pairs {
                if (a.x - b.x) * (pos.y - a.y) == (pos.x - a.x) * (a.y - b.y) {
                    antinodes += 1;
                    break;
                }
            }
        }
    }

    Ok(antinodes)
}

fn get_antenna_pairs(antennas: &[(char, IVec2)]) -> Vec<(IVec2, IVec2)> {
    antennas
        .iter()
        .flat_map(|(freq, pos)| {
            antennas
                .iter()
                .filter_map(move |(other_freq, other_pos)| {
                    if freq != other_freq || pos == other_pos {
                        return None;
                    }
                    Some((*pos, *other_pos))
                })
                .collect_vec()
        })
        .collect_vec()
}

fn parse_map(input: &str) -> Vec<(char, IVec2)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(y, c)| (c != '.').then_some((c, IVec2::new(x as i32, y as i32))))
        })
        .collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(34, process(input)?);
        Ok(())
    }
}
