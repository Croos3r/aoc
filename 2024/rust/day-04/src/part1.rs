use glam::U64Vec2;
use std::fmt::Debug;

#[derive(Copy, Clone)]
struct Element {
    position: U64Vec2,
    element: char,
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at [{},{}]",
            self.element, self.position.x, self.position.y
        )
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let grid = parse_grid(input);
    let side_length = input.lines().count() as u64;
    let rows = (0..side_length)
        .map(|row| {
            grid.iter()
                .filter(|e| e.position.y == row)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let cols = (0..side_length)
        .map(|col| {
            grid.iter()
                .filter(|e| e.position.x == col)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let diags1 = (0..side_length)
        .map(|row| {
            grid.iter()
                .filter(|e| e.position.x + e.position.y == row)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let diags2 = (0..side_length - 1)
        .map(|row| {
            grid.iter()
                .filter(|e| e.position.x + e.position.y == side_length + row)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let diags3 = (1..side_length)
        .map(|row| {
            grid.iter()
                .filter(|e| {
                    e.position.x as i64 - e.position.y as i64 == side_length as i64 - row as i64
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let diags4 = (0..side_length)
        .map(|row| {
            grid.iter()
                .filter(|e| e.position.x as i64 - e.position.y as i64 == -(row as i64))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(diags1
        .iter()
        .chain(diags2.iter())
        .chain(diags3.iter())
        .chain(diags4.iter())
        .chain(rows.iter())
        .chain(cols.iter())
        .map(|lines| lines.iter().map(|e| e.element).collect::<String>())
        .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
        .sum::<usize>())
}

fn parse_grid(input: &str) -> Vec<Element> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| Element {
                    position: U64Vec2::new(col as u64, row as u64),
                    element: c,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        // 0123456789
        //0MMMSXXMASM
        //1MSAMXMSMSA
        //2AMXSXMAAMM
        //3MSAMASMSMX
        //4XMASAMXAMM
        //5XXAMMXXAMA
        //6SMSMSASXSS
        //7SAXAMASAAA
        //8MAMMMXMMMM
        //9MXMXAXMASX
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        assert_eq!(18, process(input)?);
        Ok(())
    }
}
