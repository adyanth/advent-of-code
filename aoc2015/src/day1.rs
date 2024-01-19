#[aoc(day1, part1)]
pub fn part1(input: &[u8]) -> i32 {
    input.into_iter().map(|&i| match i {
        b'('=> 1,
        b')'=> -1,
        _ => 0,
    }).sum()
}
