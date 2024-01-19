type Dim = Vec<i32>;

#[aoc_generator(day2)]
pub fn convert_dims(input: &str) -> Vec<Dim> {
    input
        .lines()
        .map(|l| l.split('x').map(|i| i.parse::<i32>().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(dims: &[Dim]) -> i32 {
    dims.iter()
        .map(|dim| {
            assert!(dim.len() == 3);
            2 * (dim[0] * dim[1] + dim[1] * dim[2] + dim[2] * dim[0])   // Necessary to wrap
                + dim.iter().product::<i32>() / dim.iter().max().unwrap() // Extra
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(dims: &[Dim]) -> i32 {
    dims.iter()
        .map(|dim| {
            assert!(dim.len() == 3);
            2 * (dim.iter().sum::<i32>() - dim.iter().max().unwrap())    // Wrap around
                + dim.iter().product::<i32>() //  Bow
        })
        .sum()
}
