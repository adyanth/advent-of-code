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

#[aoc(day3, part1)]
pub fn part1(input: &[u8]) -> usize {
    let mut pos = Coord{x: 0, y: 0};
    let mut houses = HashSet::new();
    houses.insert(pos.clone());
    for i in input {
        pos += i;
        houses.insert(pos.clone());
    }
    houses.len()
}
