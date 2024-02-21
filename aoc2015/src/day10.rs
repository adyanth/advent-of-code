const LUT: *const u8 = b"0123456789".as_ptr();

fn looksay_mapper(mut acc: (String, char, usize), c: char) -> (String, char, usize) {
    if c == acc.1 {
        acc.2 += 1;
    } else {
        if acc.1 != ' ' {
            unsafe {
                acc.0.push(*LUT.offset(acc.2 as isize) as char);
            }
            acc.0.push(acc.1);
        }
        acc.1 = c;
        acc.2 = 1
    }
    acc
}

fn looksay(input: &str, count: usize) -> String {
    let mut output = input.to_string();
    for _ in 1..=count {
        let calc = output
            .chars()
            .fold((String::with_capacity(8388608), ' ', 0), looksay_mapper);
        output = calc.0;
        unsafe {
            output.push(*LUT.offset(calc.2 as isize) as char);
        }
        output.push(calc.1);
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
