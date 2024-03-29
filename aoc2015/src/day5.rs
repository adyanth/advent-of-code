use std::collections::HashMap;

fn is_vowel(c: &u8) -> bool {
    match *c as char {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn notnice_check(p: &u8, c: &u8) -> bool {
    c - p == 1
        && match *p as char {
            'a' | 'c' | 'p' | 'x' => true,
            _ => false,
        }
}

fn is_nice_part1(input: &str) -> u16 {
    let mut nice = false;
    let mut notnice = false;
    let mut input = input.as_bytes().iter();
    let mut prev: &u8 = input.next().unwrap();
    let mut vcount: u8 = is_vowel(prev) as u8;
    for c in input {
        vcount += is_vowel(c) as u8;
        nice = nice || prev == c;
        notnice = notnice || notnice_check(prev, c);
        prev = c;
    }
    (nice && !notnice && vcount >= 3) as u16
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u16 {
    input.lines().map(is_nice_part1).sum()
}

#[aoc(day5, part1, looper)]
pub fn part1_looper(input: &str) -> u16 {
    let mut nices = 0;
    for input in input.lines() {
        let mut nice = false;
        let mut notnice = false;
        let mut input = input.as_bytes().iter();
        let mut prev: &u8 = input.next().unwrap();
        let mut vcount: u8 = is_vowel(prev) as u8;
        for c in input {
            vcount += is_vowel(c) as u8;
            nice = nice || prev == c;
            notnice = notnice || notnice_check(prev, c);
            prev = c;
        }
        nices += (nice && !notnice && vcount >= 3) as u16;
    }
    nices
}

// cat input/2015/day5.txt | grep -E '(..).*\1' | grep -E '(.).\1' | wc -l
fn is_nice_part2(input: &str) -> u16 {
    let mut doubles = HashMap::new();
    let mut cond1 = false;
    let mut cond2 = false;

    let couple = &input[0..=1];
    doubles.insert(couple, 1);

    for i in 2..input.len() {
        let couple = &input[i - 1..=i];
        if !cond1 {
            match doubles.get(couple) {
                Some(x) => cond1 = i - x > 1,
                None => _ = doubles.insert(couple, i),
            }
        }
        if !cond2 {
            cond2 = &input[i - 2..=i - 2] == &input[i..=i];
        }
    }
    (cond1 && cond2) as u16
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u16 {
    input.lines().map(is_nice_part2).sum()
}
