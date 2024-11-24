fn process_lines(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> u32 {
    let mut numbers = line.chars().filter(|c| c.is_ascii_digit());
    let first_digit = numbers.next().expect("No digits found");
    format!("{}{}", first_digit, numbers.last().unwrap_or(first_digit))
        .parse::<u32>()
        .expect("Failed to parse number")
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
    fn test_process_lines() {
        let pairs = [
            ("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet", 142),
            ("", 0),
        ];

        for (input, expected) in pairs.iter() {
            assert_eq!(process_lines(input), *expected);
        }
    }
}
