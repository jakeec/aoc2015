use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

const NORTH: char = '^';
const EAST: char = '>';
const SOUTH: char = 'v';
const WEST: char = '<';

fn main() {
    let _res = no_of_houses_receiving_present(&">");
}

fn part_1() -> usize {
    let input = fs::read_to_string("./puzzleinput.txt").expect("File not found!");
    no_of_houses_receiving_present(&input)
}

fn part_2() -> usize {
    let input = fs::read_to_string("./puzzleinput.txt").expect("File not found!");
    deliver_presents(&input)
}

fn dedup<T: Eq + Hash + Copy>(v: &mut Vec<T>) {
    // note the Copy constraint
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

fn deliver_presents(directions: &str) -> usize {
    let mut santa_directions: Vec<char> = Vec::new();
    let mut robot_directions: Vec<char> = Vec::new();
    let directions: Vec<char> = directions.chars().collect();
    let mut i = 0;
    for dir in directions {
        i += 1;
        if i % 2 == 0 {
            santa_directions.push(dir);
        } else {
            robot_directions.push(dir);
        }
    }

    let mut santa_visited = visited_coords(santa_directions);
    let mut robot_visited = visited_coords(robot_directions);
    santa_visited.append(&mut robot_visited);
    dedup(&mut santa_visited);
    santa_visited.len()
}

fn visited_coords(directions: Vec<char>) -> Vec<(isize, isize)> {
    let mut visited: Vec<(isize, isize)> = Vec::new();
    let starting_coords = (0, 0);
    visited.push(starting_coords);
    let mut current_coords = starting_coords;

    for dir in directions {
        match dir {
            NORTH => current_coords.1 += 1,
            EAST => current_coords.0 += 1,
            SOUTH => current_coords.1 -= 1,
            WEST => current_coords.0 -= 1,
            _ => panic!("Not a valid direction!"),
        }

        if !visited.contains(&current_coords) {
            visited.push(current_coords);
        }
    }

    visited
}

fn no_of_houses_receiving_present(directions: &str) -> usize {
    let directions: Vec<char> = directions.chars().collect();
    let visited = visited_coords(directions);

    visited.len()
}

#[cfg(test)]
mod no_of_houses_receiving_present {
    use super::*;

    #[test]
    fn example_1() {
        let result = no_of_houses_receiving_present(&">");
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = no_of_houses_receiving_present(&"^>v<");
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let result = no_of_houses_receiving_present(&"^v^v^v^v^v");
        assert_eq!(result, 2);
    }

    #[test]
    fn day_3_part_1() {
        let result = part_1();
        assert_eq!(result, 2565);
    }

    #[test]
    fn part_2_example_1() {
        let result = deliver_presents(&"^v");
        assert_eq!(result, 3);
    }

    #[test]
    fn part_2_example_2() {
        let result = deliver_presents(&"^>v<");
        assert_eq!(result, 3);
    }

    #[test]
    fn part_2_example_3() {
        let result = deliver_presents(&"^v^v^v^v^v");
        assert_eq!(result, 11);
    }

    #[test]
    fn day_3_part_2() {
        let result = part_2();
        assert_eq!(result, 2639);
    }
}
