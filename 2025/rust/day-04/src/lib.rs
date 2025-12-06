use itertools::Itertools;
use std::fmt::Display;

pub mod part1;
pub mod part2;

fn parse_input(input: &str) -> Vec<Roll> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| (c == '@').then_some(Roll::new(x, y)))
                .collect_vec()
        })
        .collect_vec()
}

fn find_retrievable_rolls_in_map(map: &[Roll]) -> Vec<&Roll> {
    map.iter()
        .filter(|cell| neighbors_of_cell(&map, cell).len() < 4)
        .collect_vec()
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Roll {
    position: (usize, usize),
}

impl Roll {
    fn new(x: usize, y: usize) -> Self {
        Roll { position: (x, y) }
    }

    fn x(&self) -> &usize {
        &self.position.0
    }

    fn y(&self) -> &usize {
        &self.position.1
    }
}

fn neighbors_of_cell<'a>(map: &'a [Roll], cell: &Roll) -> Vec<&'a Roll> {
    map.into_iter()
        .filter(|neighbor| {
            (neighbor.x() != cell.x() || neighbor.y() != cell.y()) && // Not current
		   	(neighbor.y().abs_diff(*cell.y()) <= 1 && neighbor.x().abs_diff(*cell.x()) <= 1)
        })
        .collect_vec()
}

impl Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Roll({}, {})", self.x(), self.y())
    }
}
