#[macro_use()]
extern crate regex;
use regex::Regex;

use std::fs;

fn main() {}

fn part_1() -> usize {
    let input = fs::read_to_string("./puzzleinput.txt").unwrap();
    let mut lights = vec![vec![0; 1000]; 1000];
    let commands: Vec<&str> = input.split("\r\n").collect();
    let command_regex = Regex::new(r"toggle|turn off|turn on").unwrap();
    let coord_regex = Regex::new(r"[0-9]+,[0-9]+").unwrap();
    for command in commands {
        let cmd = &command_regex.captures(command).unwrap()[0];
        let ss: Vec<&str> = command.split("through").collect();
        let start = &coord_regex.captures(ss[0]).unwrap()[0];
        let end = &coord_regex.captures(ss[1]).unwrap()[0];
        let start: Vec<usize> = start
            .split(",")
            .map(|o| o.parse::<usize>().unwrap())
            .collect();
        let start = (start[0], start[1]);
        let end: Vec<usize> = end
            .split(",")
            .map(|o| o.parse::<usize>().unwrap())
            .collect();
        let end = (end[0], end[1]);

        match cmd {
            "toggle" => toggle_lights(&mut lights, start, end),
            "turn on" => turn_on(&mut lights, start, end),
            "turn off" => turn_off(&mut lights, start, end),
            _ => panic!("Not a valid instruction!"),
        }
    }

    count_lit(&lights)
}

fn part_2() -> usize {
    let input = fs::read_to_string("./puzzleinput.txt").unwrap();
    let mut lights = vec![vec![0; 1000]; 1000];
    let commands: Vec<&str> = input.split("\r\n").collect();
    let command_regex = Regex::new(r"toggle|turn off|turn on").unwrap();
    let coord_regex = Regex::new(r"[0-9]+,[0-9]+").unwrap();
    for command in commands {
        let cmd = &command_regex.captures(command).unwrap()[0];
        let ss: Vec<&str> = command.split("through").collect();
        let start = &coord_regex.captures(ss[0]).unwrap()[0];
        let end = &coord_regex.captures(ss[1]).unwrap()[0];
        let start: Vec<usize> = start
            .split(",")
            .map(|o| o.parse::<usize>().unwrap())
            .collect();
        let start = (start[0], start[1]);
        let end: Vec<usize> = end
            .split(",")
            .map(|o| o.parse::<usize>().unwrap())
            .collect();
        let end = (end[0], end[1]);

        match cmd {
            "toggle" => increase_brightness(&mut lights, start, end, 2),
            "turn on" => increase_brightness(&mut lights, start, end, 1),
            "turn off" => decrease_brightness(&mut lights, start, end),
            _ => panic!("Not a valid instruction!"),
        }
    }

    count_lit(&lights)
}

fn count_lit(lights: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for row in lights {
        for light in row {
            count += light
        }
    }
    count
}

fn increase_brightness(
    lights: &mut Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
    brightness: usize,
) {
    for x in start.0..end.0 + 1 {
        for y in start.1..end.1 + 1 {
            lights[y][x] += brightness;
        }
    }
}

fn decrease_brightness(lights: &mut Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) {
    for x in start.0..end.0 + 1 {
        for y in start.1..end.1 + 1 {
            if lights[y][x] > 0 {
                lights[y][x] -= 1;
            }
        }
    }
}

fn turn_on(lights: &mut Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) {
    for x in start.0..end.0 + 1 {
        for y in start.1..end.1 + 1 {
            lights[y][x] = 1;
        }
    }
}

fn turn_off(lights: &mut Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) {
    for x in start.0..end.0 + 1 {
        for y in start.1..end.1 + 1 {
            lights[y][x] = 0;
        }
    }
}

fn toggle_lights(lights: &mut Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) {
    for x in start.0..end.0 + 1 {
        for y in start.1..end.1 + 1 {
            match lights[y][x] {
                0 => lights[y][x] = 1,
                1 => lights[y][x] = 0,
                _ => panic!("Light state must be 0 or 1"),
            }
        }
    }
}

#[cfg(test)]
mod day_6 {
    use super::*;

    #[test]
    fn example_1() {
        let mut lights = vec![vec![0; 1000]; 1000];
        toggle_lights(&mut lights, (0, 0), (999, 999));
        assert_eq!(count_lit(&lights), 1_000_000);
    }

    #[test]
    fn example_2() {
        let mut lights = vec![vec![0; 1000]; 1000];
        toggle_lights(&mut lights, (0, 0), (999, 0));
        assert_eq!(count_lit(&lights), 1_000);
    }

    #[test]
    fn example_3() {
        let mut lights = vec![vec![1; 1000]; 1000];
        toggle_lights(&mut lights, (499, 499), (500, 500));
        assert_eq!(count_lit(&lights), 999_996);
    }

    #[test]
    fn day_6_part_1() {
        let result = part_1();
        assert_eq!(result, 543903)
    }

    #[test]
    fn day_6_part_2() {
        let result = part_2();
        assert_eq!(result, 14687245);
    }
}
