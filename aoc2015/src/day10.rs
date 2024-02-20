fn looksay_mapper(mut acc: (String, char, usize), c: char) -> (String, char, usize) {
    if c == acc.1 {
        acc.2 += 1;
    } else {
        if acc.1 != ' ' {
            acc.0.push_str(&format!("{}{}", acc.2, acc.1));
        }
        acc.1 = c;
        acc.2 = 1
    }
    acc
}

fn looksay(input: &str, count: usize) -> String {
    let mut output = input.to_string();
    for _ in 1..=count {
        let calc = output.chars().fold((String::new(), ' ', 0), looksay_mapper);
        output = format!("{}{}{}", calc.0, calc.2, calc.1);
    }
    output
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    looksay(input, 40).len()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    looksay(input, 50).len()
}
