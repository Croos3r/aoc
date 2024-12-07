use glam::IVec2;
use miette::miette;
use std::collections::HashSet;

const DIRECTIONS: [IVec2; 4] = [
    IVec2::new(-1, 0),
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(0, -1),
];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let map_size = input
        .lines()
        .next()
        .ok_or_else(|| miette!("No input"))?
        .len() as i32;
    let ((initial_guard_pos, mut guard_direction), obstacles) = parse_map(input);

    let mut visited: HashSet<IVec2> = HashSet::new();

    let mut cur_pos = initial_guard_pos;
    visited.insert(cur_pos);

    while cur_pos.x >= 0 && cur_pos.y >= 0 && cur_pos.x < map_size && cur_pos.y < map_size {
        cur_pos += guard_direction;
        visited.insert(cur_pos);
        let next_pos = cur_pos + guard_direction;

        if obstacles.contains(&next_pos) {
            let initial_direction_idx = DIRECTIONS
                .iter()
                .position(|&d| d == guard_direction)
                .unwrap();
            for i in 0..4 {
                let direction = DIRECTIONS[(initial_direction_idx + i) % 4];
                let new_pos = cur_pos + direction;
                if !obstacles.contains(&new_pos) {
                    guard_direction = direction;
                    break;
                }
            }
        }
    }

    Ok(visited.len() - 1)
}

pub fn parse_map(input: &str) -> ((IVec2, IVec2), Vec<IVec2>) {
    let mut obstacles = vec![];
    let mut guard = (IVec2::new(0, 0), IVec2::new(0, 0));
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let y = y as i32;
            let x = x as i32;
            let current_pos = IVec2::new(x, y);
            match c {
                '#' => obstacles.push(current_pos),
                '^' => guard = (current_pos, DIRECTIONS[0]),
                '>' => guard = (current_pos, DIRECTIONS[1]),
                'v' => guard = (current_pos, DIRECTIONS[2]),
                '<' => guard = (current_pos, DIRECTIONS[3]),
                _ => {}
            }
        }
    }
    (guard, obstacles)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        // ....#.....
        // .........#
        // ..........
        // ..#.......
        // .......#..
        // ..........
        // .#..^.....
        // ........#.
        // #.........
        // ......#...
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        assert_eq!(41, process(input)?);
        Ok(())
    }
}
