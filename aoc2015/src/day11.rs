fn alpha_increment(c: u8) -> u8 {
    match c {
        b'z' => b'a',
        b'h' | b'k' | b'n' => c + 2,
        _ => c + 1,
    }
}

fn part1_increment(input: &str) -> String {
    let mut input = input.as_bytes().to_owned();
    let mut i = 0;
    let mut skipped = false;
    // Skip large increment
    while i < input.len() {
        if skipped {
            input[i] = b'a';
        } else {
            match input[i] {
                b'i' | b'l' | b'o' => {
                    input[i] = alpha_increment(input[i]);
                    skipped = true;
                }
                _ => {}
            }
        }
        i += 1;
    }
    if !skipped {
        // general increment
        let mut i: isize = (input.len() - 1) as isize;
        while i >= 0 {
            input[i as usize] = alpha_increment(input[i as usize]);
            if input[i as usize] != b'a' {
                break;
            }
            i -= 1;
        }
    }
    String::from_utf8(input).unwrap()
}

fn part1_is_valid(input: &str) -> bool {
    let input = input.as_bytes();
    let mut c_abc = false;
    let mut c_aabb = 0;
    let mut i = 0;
    let mut skip_next = false;
    while i < input.len() - 2 {
        if !c_abc && input[i + 1] == input[i] + 1 && input[i + 2] == input[i + 1] + 1 {
            c_abc = true;
        } else if !skip_next && input[i] == input[i + 1] {
            c_aabb += 1;
            skip_next = true;
        } else if skip_next {
            skip_next = false;
        }
        i += 1;
    }
    if !skip_next && i == input.len() - 2 && input[i] == input[i + 1] {
        c_aabb += 1;
    }
    c_abc && c_aabb >= 2
}

fn part1_next_valid(input: &str) -> String {
    let mut next = part1_increment(input);
    while !part1_is_valid(&next) {
        next = part1_increment(&next);
    }
    next
}

#[aoc(day11, part1)]
fn part1(input: &str) -> String {
    part1_next_valid(input)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> String {
    part1_next_valid(&part1_next_valid(input))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_alpha_increment() {
        assert_eq!(alpha_increment(b'a'), b'b');
        assert_eq!(alpha_increment(b'z'), b'a');
        assert_eq!(alpha_increment(b'h'), b'j');
        assert_eq!(alpha_increment(b'i'), b'j');
        assert_eq!(alpha_increment(b'k'), b'm');
        assert_eq!(alpha_increment(b'l'), b'm');
        assert_eq!(alpha_increment(b'n'), b'p');
        assert_eq!(alpha_increment(b'o'), b'p');
    }

    #[test]
    fn test_part1_increment() {
        assert_eq!(part1_increment("aa"), "ab");
        assert_eq!(part1_increment("zz"), "aa");
        assert_eq!(part1_increment("ah"), "aj");
        assert_eq!(part1_increment("hh"), "hj");
        assert_eq!(part1_increment("hz"), "ja");

        assert_eq!(part1_increment("abcdfezz"), "abcdffaa");
        assert_eq!(part1_increment("ghijklmn"), "ghjaaaaa");
    }

    #[test]
    fn test_part1_is_valid() {
        assert!(!part1_is_valid("hijklmmn"));
        assert!(!part1_is_valid("abbceffg"));
        assert!(!part1_is_valid("abbcegjk"));
        assert!(!part1_is_valid("abcdefgh"));
        assert!(part1_is_valid("abcdffaa"));
        assert!(!part1_is_valid("ghijklmn"));
        assert!(part1_is_valid("ghjaabcc"));
    }

    #[test]
    fn test_part1_next_valid() {
        assert_eq!(part1_next_valid("abcdefgh"), "abcdffaa");
        assert_eq!(part1_next_valid("ghijklmn"), "ghjaabcc");
    }
}
