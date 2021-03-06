pub const TITLE: &str = "--- Day 2: Dive! ---";

use std::fs;

pub fn part_1() -> i64 {
    let result = fs::read_to_string("inputs/day2")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            let mut input = line.split_whitespace();
            (
                input.next().unwrap(),
                input.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<(&str, i64)>>()
        .iter()
        .scan(
            (0, 0),
            |(hor, ver), (direction, distance)| match direction.as_ref() {
                "forward" => {
                    *hor += distance;
                    Some((*hor, *ver))
                }
                "up" => {
                    *ver -= distance;
                    Some((*hor, *ver))
                }
                "down" => {
                    *ver += distance;
                    Some((*hor, *ver))
                }
                _ => None,
            },
        )
        .last()
        .unwrap();

    result.0 * result.1
}

pub fn part_2() -> i64 {
    let result = fs::read_to_string("inputs/day2")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            let mut input = line.split_whitespace();
            (
                input.next().unwrap(),
                input.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<(&str, i64)>>()
        .iter()
        .scan(
            (0, 0, 0),
            |(hor, ver, aim), (direction, distance)| match direction.as_ref() {
                "forward" => {
                    *hor += distance;
                    *ver += *aim * distance;

                    Some((*hor, *ver, *aim))
                }
                "up" => {
                    *aim -= distance;
                    Some((*hor, *ver, *aim))
                }
                "down" => {
                    *aim += distance;
                    Some((*hor, *ver, *aim))
                }
                _ => None,
            },
        )
        .last()
        .unwrap();

    result.0 * result.1
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 1815044);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 1739283308);
}
