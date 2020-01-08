#[macro_use()]
extern crate regex;
use regex::Regex;

use std::fs;
use std::io;
use std::str;

fn main() {
    let two = split_pair("keee");
    println!("{}", two);
}

fn part_1() -> u32 {
    let input = fs::read_to_string("./puzzleinput.txt").expect("File not found!");
    let split = input.split("\r\n");

    let mut nice_strings = 0;
    for s in split {
        let valid = nice_string(s);
        if valid {
            nice_strings += 1;
        }
    }

    nice_strings
}

fn part_2() -> u32 {
    let input = fs::read_to_string("./puzzleinput.txt").expect("File not found!");
    let split = input.split("\r\n");

    let mut nice_strings = 0;
    for s in split {
        let valid = new_nice_string(s);
        if valid {
            nice_strings += 1;
        }
    }

    nice_strings
}

fn nice_string(message: &str) -> bool {
    !contains_forbidden(message) && double_letters(message) && three_vowels(message)
}

fn new_nice_string(message: &str) -> bool {
    split_pair(message) && two_pairs(message)
}

fn contains_forbidden(message: &str) -> bool {
    let forbidden = Regex::new(r"ab|cd|pq|xy").unwrap();
    forbidden.is_match(message)
}

fn double_letters(message: &str) -> bool {
    let mut double = false;
    let chars = message.chars().collect::<Vec<char>>();
    let mut prev_char = chars[0];

    for i in 1..message.len() {
        let this_char = chars[i];
        if this_char == prev_char {
            double = true;
        } else {
            prev_char = this_char;
        }
    }

    double
}

fn three_vowels(message: &str) -> bool {
    let mut number_of_vowels = 0;
    let vowels = Regex::new(r"[aeiou]").unwrap();
    for c in message.chars() {
        if vowels.is_match(&c.to_string()) {
            number_of_vowels += 1;
        }
    }

    number_of_vowels >= 3
}

fn chunk(message: &str) -> Result<Vec<&str>, std::str::Utf8Error> {
    message
        .as_bytes()
        .chunks(2)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
}

fn two_pairs(message: &str) -> bool {
    for i in 0..message.len() as i32 {
        if message.len() as i32 - i as i32 >= 2 {
            let pair = &message[i as usize..i as usize + 2];
            let first = &message[0..i as usize];
            let second = &message[i as usize + 2..];
            if first.contains(pair) || second.contains(pair) {
                return true;
            }
        }
    }

    false
}

fn split_pair(message: &str) -> bool {
    for i in 0..message.len() {
        if message.len() - i >= 3 {
            let triplet = &message[i..i + 3];
            let chars: Vec<char> = triplet.chars().collect();
            if chars[0] == chars[2] {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod nice_string {
    use super::*;

    #[test]
    fn example_1() {
        let valid = nice_string("ugknbfddgicrmopn");
        assert_eq!(valid, true);
    }

    #[test]
    fn example_2() {
        let valid = nice_string("aaa");
        assert_eq!(valid, true);
    }

    #[test]
    fn example_3() {
        let valid = nice_string("jchzalrnumimnmhp");
        assert_eq!(valid, false);
    }

    #[test]
    fn example_4() {
        let valid = nice_string("haegwjzuvuyypxyu");
        assert_eq!(valid, false);
    }

    #[test]
    fn example_5() {
        let valid = nice_string("dvszwmarrgswjxmb");
        assert_eq!(valid, false);
    }

    #[test]
    fn day_5_part_1() {
        let nice_strings = part_1();
        assert_eq!(nice_strings, 255);
    }

    #[test]
    fn two_pairs_1() {
        let valid = two_pairs("xyxy");
        assert_eq!(valid, true);
    }

    #[test]
    fn two_pairs_2() {
        let valid = two_pairs("aabcdefgaa");
        assert_eq!(valid, true);
    }

    #[test]
    fn two_pairs_3() {
        let valid = two_pairs("aaa");
        assert_eq!(valid, false);
    }

    #[test]
    fn split_pair_1() {
        let valid = split_pair("aaa");
        assert_eq!(valid, true);
    }

    #[test]
    fn split_pair_2() {
        let valid = split_pair("aac");
        assert_eq!(valid, false);
    }

    #[test]
    fn new_nice_string_1() {
        let nice = new_nice_string("qjhvhtzxzqqjkmpb");
        assert_eq!(nice, true);
    }

    #[test]
    fn new_nice_string_2() {
        let nice = new_nice_string("xxyxx");
        assert_eq!(nice, true);
    }

    #[test]
    fn new_nice_string_3() {
        let nice = new_nice_string("uurcxstgmygtbstg");
        assert_eq!(nice, false);
    }

    #[test]
    fn new_nice_string_4() {
        let nice = new_nice_string("ieodomkazucvgmuy");
        assert_eq!(nice, false);
    }

    #[test]
    fn day_5_part_2() {
        let nice_strings = part_2();
        assert_eq!(nice_strings, 55);
    }
}
