use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u32 as nom_u32;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn get_possible_outcome_for_cube_set(set: CubeSet, games: &str) -> u32 {
    let games = parse_games(games);
    games
        .iter()
        .filter(|game| {
            game.sets.iter().all(|cube_set| {
                cube_set.red <= set.red && cube_set.green <= set.green && cube_set.blue <= set.blue
            })
        })
        .map(|game| game.id)
        .sum()
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .collect()
}

fn parse_game(game_line: &str) -> IResult<&str, Game> {
    let (game_line, (_, id, _)) = tuple((tag("Game "), nom_u32, tag(": ")))(game_line)?;
    let (game_line, sets) = parse_cube_sets(game_line)?;
    Ok((game_line, Game { id, sets }))
}

fn parse_cube_set(cube_set: &str) -> IResult<&str, CubeSet> {
    let (cube_set, cube_groups) = separated_list1(tag(", "), parse_cube_group)(cube_set)?;
    let map: HashMap<_, _> = cube_groups.into_iter().collect();
    Ok((
        cube_set,
        CubeSet {
            red: map.get("red").copied().unwrap_or(0),
            green: map.get("green").copied().unwrap_or(0),
            blue: map.get("blue").copied().unwrap_or(0),
        },
    ))
}

fn parse_cube_group(cube_group: &str) -> IResult<&str, (&str, u32)> {
    let (cube_group, (nb, color)) = separated_pair(
        nom_u32,
        tag(" "),
        alt((tag("red"), tag("green"), tag("blue"))),
    )(cube_group)?;
    Ok((cube_group, (color, nb)))
}

fn parse_cube_sets(cube_sets: &str) -> IResult<&str, Vec<CubeSet>> {
    let (cube_sets, sets) = separated_list1(tag("; "), parse_cube_set)(cube_sets)?;
    Ok((cube_sets, sets))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn test_get_possible_outcome_for_cube_set_and_games() {
        let cube_set = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected_result = 8;
        assert_eq!(
            get_possible_outcome_for_cube_set(cube_set, game),
            expected_result
        );
        let input = include_str!("input1.txt");
        let cube_set = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        let result = get_possible_outcome_for_cube_set(cube_set, input);
        assert_eq!(result, 2720);
    }
}
