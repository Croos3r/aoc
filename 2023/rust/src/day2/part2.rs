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

fn process(games: &str) -> u32 {
    let games = parse_games(games);
    games
        .iter()
        .map(|game| {
            let CubeSet { red, green, blue } = get_minimum_cube_set_for_game(game);
            red * green * blue
        })
        .sum()
}

fn get_minimum_cube_set_for_game(game: &Game) -> CubeSet {
    game.sets.iter().fold(
        CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        },
        |acc, cube_set| CubeSet {
            red: acc.red.max(cube_set.red),
            green: acc.green.max(cube_set.green),
            blue: acc.blue.max(cube_set.blue),
        },
    )
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
	use crate::day2::part2::{parse_game, process};

	#[test]
    fn test_get_minimum_cube_set_for_game() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(game).unwrap().1;
        let cube_set = super::get_minimum_cube_set_for_game(&game);
        assert_eq!(
            cube_set,
            super::CubeSet {
                red: 4,
                green: 2,
                blue: 6
            }
        );
        let input = include_str!("input2.txt");
        let result = process(input);
        assert_eq!(result, 71535);
    }
}
