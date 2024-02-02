use std::{collections::HashSet, ops};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl ops::AddAssign<&u8> for Coord {
    fn add_assign(&mut self, action: &u8) {
        match action {
            b'^' => self.y -= 1,
            b'v' => self.y += 1,
            b'<' => self.x -= 1,
            b'>' => self.x += 1,
            _ => unreachable!()
        }
    }
}

#[aoc(day3, part1, hash)]
pub fn part1_hash(input: &[u8]) -> usize {
    let mut pos = Coord{x: 0, y: 0};
    let mut houses = HashSet::new();
    houses.insert(pos.clone());
    for i in input {
        pos += i;
        houses.insert(pos.clone());
    }
    houses.len()
}

#[aoc(day3, part2)]
pub fn part2(input: &[u8]) -> usize {
    let mut santa = Coord{x: 0, y: 0};
    let mut robo = Coord{x: 0, y: 0};
    let mut houses = HashSet::new();
    houses.insert(santa.clone());
    for is in input.chunks_exact(2) {
        santa += &is[0];
        houses.insert(santa.clone());
        robo += &is[1];
        houses.insert(robo.clone());
    }
    houses.len()
}

#[aoc(day3, part1, array)]
pub fn part1_array(input: &[u8]) -> u16 {
    const SIZE: usize = 255;
    let mut houses = [false; SIZE * SIZE];
    let mut pos:usize = (SIZE - 1) / 2 * (SIZE + 1);
    let mut visited: u16 = 1;
    houses[pos] = true;
    for i in input {
        pos = match i {
            b'^' => pos - 128,
            b'v' => pos + 128,
            b'<' => pos - 1,
            b'>' => pos + 1,
            _ => unreachable!()
        };
        visited += !houses[pos] as u16;
        houses[pos] = true;
    }
    visited
}
