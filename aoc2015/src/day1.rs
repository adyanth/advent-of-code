#[aoc(day1, part1)]
pub fn part1(input: &[u8]) -> i32 {
    input.into_iter().map(|&i| match i {
        b'('=> 1,
        b')'=> -1,
        _ => 0,
    }).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u8]) -> i32 {
    input.into_iter().map(|&i| match i {
        b'('=> 1,
        b')'=> -1,
        _ => 0,
    }).enumerate().reduce(|(acci, acce), (i, e)| {
        if acci != 0 {
            (acci, acce)
        }
        else if acce + e < 0 {
            (i + 1, acce)
        } else {
            (0, acce + e)
        }
    }).unwrap().0 as i32
}
