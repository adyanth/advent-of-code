use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Action::TurnOn => "TurnOn",
                Action::TurnOff => "TurnOf",
                Action::Toggle => "Toggle",
            }
        )
    }
}

#[derive(Debug)]
pub struct Instr {
    from: Coord,
    to: Coord,
    action: Action,
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} from {} to {}", self.action, self.from, self.to)
    }
}

fn parse_pair(pair: &str) -> Coord {
    let mut split = pair.split(",");
    Coord {
        x: split.next().unwrap().parse().unwrap(),
        y: split.next().unwrap().parse().unwrap(),
    }
}

fn parse_line(line: &str) -> Instr {
    let mut tokens = line.rsplit(" ");
    let to = parse_pair(tokens.next().unwrap());
    tokens.next();
    let from = parse_pair(tokens.next().unwrap());
    Instr {
        from: from,
        to: to,
        action: match tokens.next().unwrap() {
            "toggle" => Action::Toggle,
            "off" => Action::TurnOff,
            "on" => Action::TurnOn,
            _ => unreachable!(),
        },
    }
}

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Vec<Instr> {
    input.lines().map(parse_line).collect()
}

struct State {
    grid: [[bool; 1000]; 1000],
}

impl State {
    fn count(&self) -> usize {
        self.grid
            .iter()
            .map(|e| e.iter().filter(|b| **b).count())
            .sum()
    }

    fn set(&mut self, from: &Coord, to: &Coord, value: bool) -> &mut State {
        for i in from.x..=to.x {
            for j in from.y..=to.y {
                self.grid[i][j] = value;
            }
        }
        self
    }

    fn toggle(&mut self, from: &Coord, to: &Coord) -> &mut State {
        for i in from.x..=to.x {
            for j in from.y..=to.y {
                self.grid[i][j] = !self.grid[i][j];
            }
        }
        self
    }

    fn apply(&mut self, i: &Instr) -> &mut State {
        match i.action {
            Action::TurnOn => self.set(&i.from, &i.to, true),
            Action::TurnOff => self.set(&i.from, &i.to, false),
            Action::Toggle => self.toggle(&i.from, &i.to),
        }
    }
}

#[aoc(day6, part1, naive)]
pub fn part1_naive(instrs: &[Instr]) -> usize {
    let mut state = State {
        grid: [[false; 1000]; 1000],
    };
    for i in instrs {
        state.apply(i);
    }
    state.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert!(parse("turn off 0,998 through 999,999").len() == 1);
        assert_eq!(
            parse("turn off 0,998 through 999,999")[0].to_string(),
            "TurnOf from (0, 998) to (999, 999)"
        );
    }

    #[test]
    fn test_naive() {
        let i1 = Instr {
            from: Coord { x: 0, y: 0 },
            to: Coord { x: 9, y: 9 },
            action: Action::TurnOn,
        };
        assert_eq!(part1_naive(&[i1]), 100);
    }
}
