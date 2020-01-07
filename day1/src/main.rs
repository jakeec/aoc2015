use std::env;
use std::fs;

fn main() {
    
}

fn part_1() -> (isize, isize) {
    let instructions = fs::read_to_string("./puzzleinput.txt").expect("Something went wrong reading the file");
    calculate_final_floor(&instructions)
}

fn part_2() -> isize {
    let instructions = fs::read_to_string("./puzzleinput.txt").expect("Something went wrong reading the file");
    calculate_final_floor(&instructions).1
}

fn calculate_final_floor(instructions: &str) -> (isize, isize) {
    let mut floor = 0;
    let mut index = 0;
    let mut first_basement_entry = -1;
    let instructions: Vec<char> = instructions.chars().collect();

    for instruction in  instructions {
        index = index + 1;
        match instruction {
            '(' => floor = floor + 1,
            ')' => floor = floor -1,
            _ => panic!("Invalid instruction! Instruction given: {}", instruction),
        }
        if floor == -1 && first_basement_entry == -1 {
            first_basement_entry = index;
        }
    }


    (floor, first_basement_entry)
}

#[cfg(test)]
mod calculate_final_floor {
    use super::*;

    #[test]
    fn it_goes_up_a_floor() {
        let result = calculate_final_floor(&"(");
        assert_eq!(result.0, 1);
    }

    #[test]
    fn it_goes_down_a_floor() {
        let result = calculate_final_floor(&")");
        assert_eq!(result.0, -1);
    }

    #[test]
    fn example_1() {
        let result = calculate_final_floor(&"(())");
        assert_eq!(result.0, 0);
    }

    #[test]
    fn example_2() {
        let result = calculate_final_floor(&"(((");
        assert_eq!(result.0, 3);
    }

    #[test]
    fn example_3() {
        let result = calculate_final_floor(&"))(((((");
        assert_eq!(result.0, 3);
    }

    #[test]
    fn example_4() {
        let result = calculate_final_floor(&"())");
        assert_eq!(result.0, -1);
    }

    #[test]
    fn example_5() {
        let result = calculate_final_floor(&")))");
        assert_eq!(result.0, -3);
    }

    #[test]
    fn day_1_part_1() {
        let result = part_1();
        assert_eq!(result.0, 232);
    }

    #[test]
    fn day_1_part_2() {
        let result = part_1();
        assert_eq!(result.1, 1783);
    }
}