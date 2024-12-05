type Data = Vec<Vec<char>>;

#[aoc_generator(day4)]
fn generate(input: &str) -> Data {
    input.lines().map(|line| line.chars().map(|c| match c {
        'X' | 'M' | 'A' | 'S' => c,
        _ => '.'
    }).collect()).collect()
}

fn is_e(input: &Data, x: isize, y: isize) -> i32 {
    let cols = input[0].len() as isize;
    if y + 1 >= cols {
        return 0
    }
    let i = x as usize;
    let j = y as usize;
    let found = input[i][j-1] == 'M' && input[i][j] == 'A' && input[i][j+1] == 'S';
    found as i32
}

fn is_w(input: &Data, x: isize, y: isize) -> i32 {
    if y - 1 < 0 {
        return 0
    }
    let i = x as usize;
    let j = y as usize;
    let found = input[i][j-1] == 'S' && input[i][j] == 'A' && input[i][j+1] == 'M';
    found as i32
}

fn is_n(input: &Data, x: isize, y: isize) -> i32 {
    if x - 1 < 0 {
        return 0
    }
    let i = x as usize;
    let j = y as usize;
    let found = input[i+1][j] == 'M' && input[i][j] == 'A' && input[i-1][j] == 'S';
    found as i32
}

fn is_s(input: &Data, x: isize, y: isize) -> i32 {
    let rows = input.len() as isize;
    if x + 1 >= rows {
        return 0
    }
    let i = x as usize;
    let j = y as usize;
    let found = input[i-1][j] == 'M' && input[i][j] == 'A' && input[i+1][j] == 'S';
    found as i32
}

fn is_nw(input: &Data, x: isize, y: isize) -> i32 {
    if x-1 < 0 || y-1 < 0 {
        return 0
    }
    let i = x as usize;
    let j = y as usize;
    let found = input[i+1][j+1] == 'M' && input[i][j] == 'A' && input[i-1][j-1] == 'S';
    found as i32
}

fn is_ne(input: &Data, x: isize, y: isize) -> i32 {
    let cols = input[0].len() as isize;
    if x-1 < 0 || y+1 >= cols {
        return 0
    }
    let i = x as usize;
    let j = y as usize;
    let found = input[i+1][j-1] == 'M' && input[i][j] == 'A' && input[i-1][j+1] == 'S';
    found as i32
}

fn is_se(input: &Data, x: isize, y: isize) -> i32 {
    let rows = input.len() as isize;
    let cols = input[0].len() as isize;
    if x+1 >=rows || y+1 >= cols {
        return 0
    }
    let i = x as usize;
    let j = y as usize;
    let found = input[i-1][j-1] == 'M' && input[i][j] == 'A' && input[i+1][j+1] == 'S';
    found as i32
}

fn is_sw(input: &Data, x: isize, y: isize) -> i32 {
    let rows = input.len() as isize;
    if x+1 >=rows || y-1 < 0 {
        return 0
    }
    let i = x as usize;
    let j = y as usize;
    let found = input[i-1][j+1] == 'M' && input[i][j] == 'A' && input[i+1][j-1] == 'S';
    found as i32
}

fn count(input: &Data, x: isize, y: isize) -> i32 {
    if input[x as usize][y as usize] != 'X' {
        return 0;
    }
    is_e(input, x, y + 2) +
    is_w(input, x, y - 2) +
    is_n(input, x - 2, y) +
    is_s(input, x + 2, y) +
    is_nw(input, x-2, y-2) +
    is_ne(input, x-2, y+2) +
    is_se(input, x+2, y+2) +
    is_sw(input, x+2, y-2)
}

#[aoc(day4, part1)]
pub fn part1(input: &Data) -> i32 {
    input.iter().enumerate().map(|(i, row)| 
        row.iter().enumerate().map(|(j, _)| count(input, i as isize, j as isize)).sum::<i32>()
    ).sum()
}

fn is_lr(input: &Data, x: usize, y: usize) -> bool {
    input[x-1][y-1] == input[x+1][y-1] && 
    input[x-1][y+1] == input[x+1][y+1] &&
    (
        (input[x-1][y-1] == 'M' && input[x-1][y+1] == 'S') ||
        (input[x-1][y-1] == 'S' && input[x-1][y+1] == 'M')
    )
}

fn is_ud(input: &Data, x: usize, y: usize) -> bool {
    input[x-1][y-1] == input[x-1][y+1] && 
    input[x+1][y-1] == input[x+1][y+1] &&
    (
        (input[x-1][y-1] == 'M' && input[x+1][y-1] == 'S') ||
        (input[x-1][y-1] == 'S' && input[x+1][y-1] == 'M')
    )
}

fn count_x(input: &Data, x: usize, y: usize) -> i32 {
    let rows = input.len();
    let cols = input[0].len();
    if x == 0 || x == rows - 1 || y == 0 || y == cols - 1 || input[x as usize][y as usize] != 'A' {
        return 0;
    }
    (is_lr(input, x, y) || is_ud(input, x, y)) as i32
}

#[aoc(day4, part2)]
pub fn part2(input: &Data) -> i32 {
    input.iter().enumerate().map(|(i, row)| 
        row.iter().enumerate().map(|(j, _)| count_x(input, i, j)).sum::<i32>()
    ).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = generate("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        assert_eq!(part1(&input), 18);
        assert_eq!(part2(&input), 9);
    }
}
