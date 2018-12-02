#[aoc_generator(day1)]
pub fn input_gen(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut visited = std::collections::HashSet::new();
    let mut cur = 0;
    visited.insert(cur);
    input
        .into_iter()
        .cycle()
        .find(|&i| {
            cur += i;
            !visited.insert(cur)
        }).unwrap();
    cur
}
