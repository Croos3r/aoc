use itertools::{repeat_n, Itertools};
use miette::miette;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

const OPERATORS: [Operator; 3] = [Operator::Add, Operator::Multiply, Operator::Concatenate];

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<u64> {
    let (_, equations) = parse_equations(_input).map_err(|e| miette!(e.to_string()))?;

    Ok(equations
        .par_iter()
        .filter_map(|(lhs, rhs)| solve_equation(lhs, rhs))
        .sum())
}

fn solve_equation(expected_result: &u64, rhs: &[u64]) -> Option<u64> {
    repeat_n(OPERATORS.into_iter(), rhs.len() - 1)
        .multi_cartesian_product()
        .any(|operator_combination| {
            let mut operators = operator_combination.iter();
            let iter = rhs.iter();
            iter.copied()
                .reduce(|acc, next| match operators.next().unwrap() {
                    Operator::Add => acc + next,
                    Operator::Multiply => acc * next,
                    Operator::Concatenate => format!("{}{}", acc, next).parse::<u64>().unwrap(),
                })
                .unwrap()
                == *expected_result
        })
        .then_some(*expected_result)
}

fn parse_equations(equations: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(char('\n'), parse_equation)(equations)
}

fn parse_equation(equation: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(
        nom::character::complete::u64,
        tag(": "),
        separated_list1(char(' '), nom::character::complete::u64),
    )(equation)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";
        assert_eq!(11387, process(input)?);
        Ok(())
    }
}
