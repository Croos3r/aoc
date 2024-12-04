#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let side_length = grid.len();
    let mut count = 0;
    for x in 1..(side_length - 1) {
        for y in 1..(side_length - 1) {
            if grid[x][y] == 'A' && is_x_mas(&grid, x as isize, y as isize) {
                count += 1;
            }
        }
    }
    Ok(count)
}

#[inline]
fn is_x_mas(grid: &[Vec<char>], x: isize, y: isize) -> bool {
    let diags = [((-1, -1), (1, 1)), ((-1, 1), (1, -1))];
    let mut result = true;
    for ((x0, y0), (x1, y1)) in diags {
        let diag = format!(
            "{}{}",
            grid[(x + x0) as usize][(y + y0) as usize],
            grid[(x + x1) as usize][(y + y1) as usize]
        );
        result = result && (diag == "SM" || diag == "MS");
    }
    result
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
        assert_eq!(9, process(input)?);
        Ok(())
    }
}
