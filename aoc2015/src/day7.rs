use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Op1 {
    Assign,
    Not,
}

#[derive(Clone, Debug)]
pub enum Op2 {
    And,
    Or,
    Rshift,
    Lshift,
}

#[derive(Clone, Debug)]
pub enum VarNum {
    Num(u16),
    Var(String),
}

impl VarNum {
    fn from(i: &str) -> VarNum {
        if let Ok(n) = i.parse::<u16>() {
            VarNum::Num(n)
        } else {
            VarNum::Var(i.to_string())
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    UnOp(Op1, VarNum),          // Op1, src, dst
    BinOp(Op2, VarNum, VarNum), // Op2, src1, src2, dst
}

#[derive(Clone, Debug)]
pub struct Context {
    env: HashMap<String, Expr>,
    cache: HashMap<String, u16>,
}

fn parse(input: &str, context: &mut Context) {
    // println!("Parsing {}", input);
    let mut expr = input.split(" -> ");
    let lhs: Vec<&str> = expr.next().unwrap().split(" ").collect();
    let rhs = expr.next().unwrap();
    context.env.insert(
        rhs.to_string(),
        match lhs.len() {
            1 => Expr::UnOp(Op1::Assign, VarNum::from(lhs[0])),
            2 => Expr::UnOp(Op1::Not, VarNum::from(lhs[1])), // NOT only
            3 => Expr::BinOp(
                match lhs[1] {
                    "AND" => Op2::And,
                    "OR" => Op2::Or,
                    "LSHIFT" => Op2::Lshift,
                    "RSHIFT" => Op2::Rshift,
                    _ => unimplemented!(),
                },
                VarNum::from(lhs[0]),
                VarNum::from(lhs[2]),
            ),
            _ => unimplemented!(),
        },
    );
}

fn evaluate_varnum(x: &VarNum, context: &mut Context) -> u16 {
    // println!("Evaluating {:?}", x);
    match x {
        VarNum::Num(n) => *n,
        VarNum::Var(x) => {
            if let Some(val) = context.cache.get(x) {
                *val
            } else {
                let val = evaluate_expr(&context.env.get(x).unwrap().clone(), context);
                context.cache.insert(x.to_string(), val);
                val
            }
        }
    }
}

fn evaluate_expr(expr: &Expr, context: &mut Context) -> u16 {
    // println!("Evaluating {:?}", expr);
    match expr {
        Expr::UnOp(op, src) => {
            let src = evaluate_varnum(src, context);
            match op {
                Op1::Assign => src,
                Op1::Not => !src,
            }
        }
        Expr::BinOp(op, src1, src2) => {
            let src1 = evaluate_varnum(src1, context);
            let src2 = evaluate_varnum(src2, context);
            match op {
                Op2::And => src1 & src2,
                Op2::Or => src1 | src2,
                Op2::Lshift => src1 << src2,
                Op2::Rshift => src1 >> src2,
            }
        }
    }
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Context {
    let mut c = Context {
        env: HashMap::new(),
        cache: HashMap::new(),
    };
    for line in input.lines() {
        parse(line, &mut c);
    }
    c
}

fn solve_for(context: &mut Context, var: &str) -> u16 {
    evaluate_expr(&context.env.get(var).unwrap().clone(), context)
}

#[aoc(day7, part1)]
pub fn part1(context: &Context) -> u16 {
    solve_for(&mut context.clone(), "a")
}

#[aoc(day7, part2)]
pub fn part2(context: &Context) -> u16 {
    let a = solve_for(&mut context.clone(), "a");
    let mut context = context.clone();
    context.cache.insert("b".to_string(), a);
    solve_for(&mut context, "a")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(part1(&parse_input("123 -> a")), 123);
    }

    #[test]
    fn test_two() {
        assert_eq!(part1(&parse_input("123 -> b\nb -> a")), 123);
    }

    #[test]
    fn test_complex() {
        let parsed = &mut parse_input(
            "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i",
        );
        assert_eq!(solve_for(parsed, "d"), 72);
        assert_eq!(solve_for(parsed, "e"), 507);
        assert_eq!(solve_for(parsed, "f"), 492);
        assert_eq!(solve_for(parsed, "g"), 114);
        assert_eq!(solve_for(parsed, "h"), 65412);
        assert_eq!(solve_for(parsed, "i"), 65079);
        assert_eq!(solve_for(parsed, "x"), 123);
        assert_eq!(solve_for(parsed, "y"), 456);
    }
}
