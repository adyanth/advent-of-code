use serde_json::Value;
use serde_json::Value::*;

#[aoc_generator(day12)]
fn parse(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}
fn recurse(f: &mut dyn FnMut(i64), value: &Value) {
    match value {
        Null => {}
        Bool(_) => {}
        Number(n) => f(n.as_i64().unwrap()),
        String(_) => {}
        Array(v) => v.iter().map(|x| recurse(f, x)).for_each(drop),
        Object(kv) => {
            kv.keys().map(|x| f(x.parse().unwrap_or_default())).for_each(drop);
            kv.values().map(|x| recurse(f, x)).for_each(drop);
        }
    }
}

#[aoc(day12, part1)]
fn part1(input: &Value) -> i64 {
    let mut c = 0;
    recurse(&mut |x| c = c + x, input);
    c
}
