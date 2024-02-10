use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct Coord {
    x: i32,
    y: i32,
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
    Instr{
        from: from,
        to: to,
        action: match tokens.next().unwrap() {
            "toggle" => Action::Toggle,
            "off" => Action::TurnOff,
            "on" => Action::TurnOn,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Vec<Instr> {
    input.lines().map(parse_line).collect()
}

#[aoc(day6, part1)]
pub fn part1(instrs: &[Instr]) -> i64 {
    0
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
}
