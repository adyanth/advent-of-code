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
    println!("Chars: `{:?}`", chars);
    chars.as_str()
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    println!("Input: `{}`", input);
    input.lines().map(strip).map(count).sum()
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
                r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#
            ),
            12
        );
    }
}
