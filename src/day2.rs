use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn checksum(v: &[char]) -> (i32, i32) {
    let mut counts: HashMap<char, i32> = HashMap::new();
    for &c in v {
        *counts.entry(c).or_default() += 1;
    }
    (
        counts.values().any(|&v| v == 2) as i32,
        counts.values().any(|&v| v == 3) as i32,
    )
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<char>]) -> i32 {
    let mut tot_2 = 0;
    let mut tot_3 = 0;
    for i in input {
        let (_2, _3) = checksum(i);
        tot_2 += _2;
        tot_3 += _3;
    }
    tot_2 * tot_3
}

fn hamming(v1: &[char], v2: &[char]) -> i32 {
    v1.into_iter()
        .zip(v2.into_iter())
        .fold(0, |d, (c1, c2)| d + (c1 != c2) as i32)
}
#[aoc(day2, part2)]
pub fn part2(input: &[Vec<char>]) -> String {
    for i in 0..input.len() {
        for j in i..input.len() {
            if hamming(&input[i], &input[j]) == 1 {
                return input[i]
                    .iter()
                    .zip(input[j].iter())
                    .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
                    .collect();
            }
        }
    }
    panic!("no pair");
}
