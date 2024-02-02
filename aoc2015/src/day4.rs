use md5;

#[aoc(day4, part1, base)]
pub fn part1_base(input: &str) -> i32 {
    let mut i = 1;
    let mut hash = md5::compute(format!("{input}{i}")).0;
    while !hash.starts_with(&[0, 0]) || hash[2] >= 16 {
        i += 1;
        hash = md5::compute(format!("{input}{i}")).0;
    }
    i
}

#[aoc(day4, part1, optimized)]
pub fn part1_optimized(input: &str) -> i32 {
    let mut i = 1;
    let il = input.len();
    let mut c = String::with_capacity(il + 7);
    c.clone_from(&input.to_string());
    c.insert_str(il, &i.to_string());
    let mut hash = md5::compute(&c).0;
    while !hash.starts_with(&[0, 0]) || hash[2] >= 16 {
        i += 1;
        c.truncate(il);
        c.insert_str(il, &i.to_string());
        hash = md5::compute(&c).0;
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
