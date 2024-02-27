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

fn recurse2(f: &mut dyn FnMut(i64), value: &Value) {
    match value {
        Null => {}
        Bool(_) => {}
        Number(n) => f(n.as_i64().unwrap()),
        String(_) => {}
        Array(v) => v.iter().map(|x| recurse2(f, x)).for_each(drop),
        Object(kv) => {
            if !kv.values().any(|x| x.as_str() == Some("red")) {
                kv.keys().map(|x| f(x.parse().unwrap_or_default())).for_each(drop);
                kv.values().map(|x| recurse2(f, x)).for_each(drop);
            }
        }
    }
}

#[aoc(day12, part2)]
fn part2(input: &Value) -> i64 {
    let mut c = 0;
    recurse2(&mut |x| c = c + x, input);
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red() {
        recurse2(&mut |x| println!("{}", x), &serde_json::from_str(r#"{"e":"violet","c":"red","a":"blue","b":-22,"d":[71,"red",30,"violet","red",26,120],"f":["red"]}"#).unwrap());
    }
}