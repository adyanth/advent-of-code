#[derive(Debug)]
struct Data {
    rules: Vec<(i32, i32)>,
    pagesets: Vec<Vec<i32>>,
}

#[aoc_generator(day5)]
fn generate(input: &str) -> Data {
    let mut rules = vec![];
    let mut pagesets = vec![];
    let mut parsing_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue
        }
        if parsing_rules {
            let splits = line.split("|").collect::<Vec<&str>>();
            let [a,b] = splits.as_slice() else { todo!() };
            rules.push((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            pagesets.push(line.split(",").map(|e| e.parse().unwrap()).collect());
        }
    }
    Data {
        rules: rules,
        pagesets: pagesets,
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &Data) -> i32 {
    dbg!("{:?}", input);
    0
}

// #[aoc(day5, part2)]
// pub fn part2(input: &Data) -> i32 {
//     0
// }
