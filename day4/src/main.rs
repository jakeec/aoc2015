#[macro_use()]
extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::f64;
use std::str;

fn main() {
    let result = part_1(&"abcdef");
    println!("{}", result);
}

fn part_1(secret_key: &str) -> u32 {
    let mut found = false;
    let mut num = 0;
    let mut final_result: &str = "";
    while !found {
        let result = md5crate(&format!("{}{}", secret_key, num));
        if &result[..5] == "00000" {
            final_result = &result;
            found = true;
        }
        if num % 10000 == 0 {
            println!("{}", num);
        }
        num += 1;
    }

    return num - 1;
}

fn part_2(secret_key: &str) -> u32 {
    let mut found = false;
    let mut num = 0;
    let mut final_result: &str = "";
    while !found {
        let result = md5crate(&format!("{}{}", secret_key, num));
        if &result[..6] == "000000" {
            final_result = &result;
            found = true;
        }
        if num % 10000 == 0 {
            println!("{}", num);
        }
        num += 1;
    }

    return num - 1;
}

fn chunk(message: &str, size: usize) -> Vec<&str> {
    message
        .as_bytes()
        .chunks(size)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
}

fn leftrotate(x: usize, c: usize) -> usize {
    ((x << c) | (x >> (32 - c)))
}

fn md5crate(message: &str) -> String {
    let mut hash = Md5::new();
    hash.input_str(message);
    let result = hash.result_str();
    result
}

fn md5(message: &str) {
    let message: Vec<String> = message
        .as_bytes()
        .into_iter()
        .map(|b| format!("{:b}", b))
        .collect();
    let mut message: String = message.join("");
    let mut s: Vec<u32> = Vec::new();
    let mut k: Vec<u32> = vec![0; 64];
    s.append(&mut vec![
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
    ]);
    s.append(&mut vec![
        5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
    ]);
    s.append(&mut vec![
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
    ]);
    s.append(&mut vec![
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ]);

    let two: f64 = 2.0;

    for i in 0..63 {
        k[i] = f64::floor(two.powf(32.0) * f64::abs(f64::sin((i + 1) as f64))) as u32;
    }

    let a0: u32 = 0x67452301;
    let b0: u32 = 0xefcdab89;
    let c0: u32 = 0x98badcfe;
    let d0: u32 = 0x10325476;

    let original_length = message.len();

    message += "1";

    while (message.len() % 512) != 448 {
        message += "0";
    }

    let orig = &format!("{:b}", (original_length as f64 % two.powf(64.0)) as usize);
    let orig = &format!("{:0<64}", orig);
    message += orig;

    let chunks: Vec<&str> = chunk(&mut message, 512);

    for c in chunks {
        let M: Vec<&str> = chunk(c, 32);

        let mut a = a0 as usize;
        let mut b = b0 as usize;
        let mut c = c0 as usize;
        let mut d = d0 as usize;

        for i in 0..63 {
            let mut F: usize;
            let mut g: usize;

            if i >= 0 && i <= 15 {
                F = ((b & c) | ((!b) & d)) as usize;
                g = ((5 * i + 1) % 16) as usize;
            } else if i >= 16 && i <= 31 {
                F = ((d & b) | ((!d) & c)) as usize;
                g = ((5 * i + 1) % 16) as usize;
            } else if i >= 32 && i <= 47 {
                F = (b ^ c ^ d) as usize;
                g = ((3 * i + 5) % 16) as usize;
            } else if i >= 48 && i <= 63 {
                F = (c ^ (b | (!d))) as usize;
                g = ((7 * i) % 16) as usize;
            } else {
                panic!("Something went terribly wrong!");
            }

            println!("F: {}", F);
            println!("a: {}", a);
            println!("K[i]: {}", k[i as usize]);
            println!("M[g]: {:?}", u32::from_str_radix(M[g as usize], 2).unwrap());

            F = F
                + a as usize
                + k[i as usize] as usize
                + u32::from_str_radix(M[g as usize], 2).unwrap() as usize;
            a = d;
            d = c;
            let temp = b;
            c = temp;
            println!("{}", leftrotate(F, s[i] as usize));
            b = temp + leftrotate(F, s[i] as usize);
        }
    }
}

#[cfg(test)]
mod md5 {
    use super::*;

    #[test]
    fn example_1() {
        let result = part_1(&"abcdef");
        assert_eq!(result, 609043);
    }

    #[test]
    fn example_2() {
        let result = part_1(&"pqrstuv");
        assert_eq!(result, 1048970);
    }

    #[test]
    fn day_4_part_1() {
        let result = part_1(&"bgvyzdsv");
        assert_eq!(result, 254575);
    }

    #[test]
    fn day_4_part_2() {
        let result = part_2(&"bgvyzdsv");
        assert_eq!(result, 1038736);
    }
}
