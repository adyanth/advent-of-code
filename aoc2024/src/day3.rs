use regex::Regex;
type Data = String;

#[aoc_generator(day3)]
fn generate(input: &str) -> Data {
    input.to_string()
}

#[aoc(day3, part1)]
pub fn part1(input: &Data) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re
        .captures_iter(input.as_str())
        .map(|e| e.extract())
        .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &Data) -> i32 {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for e in re.captures_iter(input) {
        enabled = e
                    .name("do")
                    .map_or(
                        e.name("dont")
                        .map_or(
                            enabled, 
                            |_| false
                        ),
                        |_| true
                    );
        sum += 
            enabled as i32
            * e.name("a").map_or(0, |v| v.as_str().parse().unwrap())
            * e.name("b").map_or(0, |v| v.as_str().parse().unwrap())
    }
    sum
}
