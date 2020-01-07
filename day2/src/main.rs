use std::fs;

fn main() {
    println!("Hello, world!");
}

fn part_1() -> usize {
    let input = fs::read_to_string("./puzzleinput.txt").expect("File not found!");
    let split = input.split("\r\n");
    let mut total = 0;
    for s in split {
        let dimensions: Vec<usize> = s.split('x').map(|o| o.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        total += calculate_wrapping_paper(dimensions[0], dimensions[1], dimensions[2]);
    }
    total
}

fn part_2() -> usize {
    let input = fs::read_to_string("./puzzleinput.txt").expect("File not found!");
    let split = input.split("\r\n");
    let mut total = 0;
    for s in split {
        let dimensions: Vec<usize> = s.split('x').map(|o| o.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        total += calculate_ribbon(dimensions[0], dimensions[1], dimensions[2]);
    }
    total
}

fn calculate_wrapping_paper(l: usize, w: usize, h: usize) -> usize {
    let a = l*w;
    let b = w*h;
    let c = h*l;
    let faces = &[a, b, c];
    let mut smallest_face = a;
    for face in faces {
        if face < &smallest_face {
            smallest_face = *face;
        }
    }
    2*a + 2*b + 2*c + (smallest_face as usize)
}

fn calculate_ribbon(l: usize, w: usize, h: usize) -> usize {
    let mut dimensions = vec![l, w, h];
    dimensions.sort();
    let a = dimensions[0];
    let b = dimensions[1];
    let length = a*2 + b*2;
    let bow_length = l*w*h;
    length + bow_length
}

#[cfg(test)]
mod calculate_wrapping_paper {
    use super::*;

    #[test]
    fn example_1() {
        let result = calculate_wrapping_paper(2, 3, 4);
        assert_eq!(result, 58);
    }

    #[test]
    fn example_2() {
        let result = calculate_wrapping_paper(1,1,10);
        assert_eq!(result, 43);
    }

    #[test]
    fn day_2_part_1() {
        let result = part_1();
        assert_eq!(result, 1588178);
    }
}

#[cfg(test)]
mod calculate_ribbon {
    use super::*;

    #[test]
    fn example_1() {
        let result = calculate_ribbon(2, 3, 4);
        assert_eq!(result, 34);
    }

    #[test]
    fn example_2() {
        let result = calculate_ribbon(1, 1, 10);
        assert_eq!(result, 14);
    }

    #[test]
    fn day_2_part_2() {
        let result = part_2();
        assert_eq!(result, 3783758);
    }
}

