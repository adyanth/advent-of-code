use std::ops::Index;

fn count(line: &str) -> usize {
    let res = 2 + line
        .to_ascii_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' => 0,
            '"' | '\\' => 1,
            _ => 3,
        })
        .sum::<usize>();
    println!("`{line}` -> {res}");
    res
}

fn strip(line: &str) -> &str {
    let mut chars = line.chars();
    chars.next();
    chars.next_back();
    // println!("Chars: `{:?}`", chars);
    chars.as_str()
}

fn count_correct(line: &str) -> usize {
    let bytes = line.as_bytes();
    let mut i = 0;
    let mut count = 2;
    while i < bytes.len() {
        if *bytes.index(i) == b'\\' {
            match bytes.index(i+1) {
                b'x' => {
                    count += 3;
                    i += 3;
                },
                b'"' | b'\\' => {
                    count += 1;
                    i += 1;
                },
                _ => unreachable!(),
            }
        }
        i += 1;
    }
    count
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    // println!("Input: `{}`", input);
    input.lines().map(strip).map(count_correct).sum()
}

fn count_part2(line: &str) -> usize {
    2 + line.bytes().map(|c| match c {
        b'"' | b'\\' => 1,
        _ => 0,
    }).sum::<usize>()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    // println!("Input: `{}`", input);
    input.lines().map(count_part2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(part1(r#""""#), 2);
        assert_eq!(part1(r#""abc""#), 2);
        assert_eq!(part1(r#""aaa\"aaa""#), 3);
        assert_eq!(part1(r#""\x27""#), 5);
        assert_eq!(part1(r#""\\\\""#), 4);
    }

    #[test]
    fn test_all() {
        assert_eq!(
            part1(
                r#"""
"abc"
"aaa\"aaa"
"\x27"
"#
            ),
            12
        );
    }
}
