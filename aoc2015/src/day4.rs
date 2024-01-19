use md5;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let mut i = 1;
    let mut hash = md5::compute(format!("{input}{i}")).0;
    while !hash.starts_with(&[0, 0]) || hash[2] >= 16 {
        i += 1;
        hash = md5::compute(format!("{input}{i}")).0;
    }
    i
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let mut i = 1;
    let mut hash = md5::compute(format!("{input}{i}")).0;
    while !hash.starts_with(&[0, 0, 0]) {
        i += 1;
        hash = md5::compute(format!("{input}{i}")).0;
    }
    i
}
