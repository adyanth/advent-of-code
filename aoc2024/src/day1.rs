use std::collections::HashMap;

use itertools::Itertools;
pub struct Data {
    a: Vec<i32>,
    b: Vec<i32>,
}

#[aoc_generator(day1)]
fn generate(input: &str) -> Data {
    let mut ina = Vec::new();
    let mut inb = Vec::new();
    input.lines().for_each(|line| {
        let (a, b) = line.split_once("   ").unwrap();
        ina.push(a.parse().unwrap());
        inb.push(b.parse().unwrap());
    });

    Data {
        a: ina,
        b: inb,
    }
}

#[aoc(day1, part1)]
pub fn part1(input: &Data) -> i32 {
    input.a.clone().into_iter().sorted()
        .zip(input.b.clone().into_iter().sorted())
        .fold(0, |acc, (ia, ib)| acc + (ib - ia).abs())
}

#[aoc(day1, part2)]
pub fn part2(input: &Data) -> i32 {
    let mut freqb = HashMap::new();
    for ele in &input.b {
        freqb.insert(ele, freqb.get(ele).unwrap_or(&0) + 1);
    }
    input.a.iter().map(|e| e * freqb.get(e).unwrap_or(&0)).sum()
}
