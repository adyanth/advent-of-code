pub type Data = Vec<Vec<i32>>;

#[aoc_generator(day2)]
fn generate(input: &str) -> Data {
    let mut data = Vec::new();
    input.lines().for_each(|line| {
        data.push(line.split_whitespace().map(|e| e.parse().unwrap()).collect());
    });

    data
}

fn diff<'a>(vec: &'a Vec<i32>) -> impl Iterator<Item=i32> + 'a {
    vec.iter().zip(vec.iter().skip(1)).map(|(a,b)| a - b)
}

fn check_report(report: &Vec<i32>) -> bool {
    diff(report).map(|e| match e {
        i if i.abs() >= 1 && i.abs() <= 3 => i.signum(),
        _ => 0
    }).sum::<i32>().abs() == (report.len() - 1) as i32
}

fn skip_any_check(report: &Vec<i32>) -> bool {
    (0..report.len()).any(
        |i| check_report(
            &report
            .iter()
            .enumerate()
            .filter(|(e, _)| e != &i)
            .map(|(_, v)| *v)
            .collect()
        )
    )
}

fn check_report2(report: &Vec<i32>) -> bool {
    check_report(report) || skip_any_check(report)
}

#[aoc(day2, part1)]
pub fn part1(input: &Data) -> i32 {
    input.iter().map(check_report).map(|e| e as i32).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &Data) -> i32 {
    input.iter().map(check_report2).map(|e| e as i32).sum()
}
