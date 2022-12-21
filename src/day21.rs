use std::collections::BTreeMap;
use std::io::stdin;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Multiply,
    Add,
    Subtract,
    Divide,
}
use Operation::{*};

impl Operation {
    fn eval(&self, lhs: Option<i64>, rhs: Option<i64>) -> Option<i64> {
        let lhs = lhs?;
        let rhs = rhs?;

        Some(match self {
            Multiply => lhs * rhs,
            Add => lhs + rhs,
            Divide => lhs / rhs,
            Subtract => lhs - rhs,
        })
    }

    fn recover_operand(&self, lhs: Option<i64>, rhs: Option<i64>, target: i64) -> i64 {
        assert_ne!(lhs.is_some(), rhs.is_some());

        return match self {
            Add => target - lhs.unwrap_or_default() - rhs.unwrap_or_default(),
            Multiply => {let result = target / lhs.unwrap_or(1) / rhs.unwrap_or(1);
                assert_eq!(target, lhs.unwrap_or(result) * rhs.unwrap_or(result));
                result}
            Subtract => {
                // target = lhs - rhs
                if rhs.is_none() {
                    lhs.unwrap() - target
                } else {
                    target + rhs.unwrap()
                }
            }
            Divide => {
                // target = lhs / rhs
                let res = if rhs.is_none() {
                    lhs.unwrap() / target
                } else {
                    target * rhs.unwrap()
                };
                assert_eq!(target, lhs.unwrap_or(res) / rhs.unwrap_or(res));
                return res;
            }
        };
    }
}

#[derive(Debug)]
enum Monkey {
    Number(i64),
    Operator(String, String, Operation),
    Undefined,
}
use Monkey::{*};

fn parse_monkey(s: &str) -> Monkey {
    let s_int = s.parse::<i64>();
    if s_int.is_ok() {
        return Number(s_int.unwrap());
    }

    let parts: [&str; 3] = s.split(" ").collect::<Vec<&str>>().try_into().unwrap();

    let op = match parts[1] {
        "*" => Multiply,
        "+" => Add,
        "-" => Subtract,
        "/" => Divide,
        _ => panic!()
    };

    return Operator(String::from(parts[0].trim()),
                    String::from(parts[2].trim()),
                    op);
}

fn parse_input() -> BTreeMap<String, Monkey> {
    return stdin().lines()
        .map(|p| p.unwrap())
        .map(|p| {
            let p = p.split_once(":").unwrap();
            return (String::from(p.0.trim()), parse_monkey(p.1.trim()));
        })
        .collect();
}

pub(crate) fn main() {
    let mut monkeys = parse_input();
    let mut cache = BTreeMap::new();

    println!("[easy] root: {}", calculate(&monkeys, "root", &mut cache).unwrap());

    monkeys.insert(String::from("humn"), Undefined);
    cache.clear();
    let (root_l, root_r) = match monkeys.get("root").unwrap() {
        Operator(lhs, rhs, _) => (&lhs[..], &rhs[..]),
        _ => panic!()
    };

    let f_root_l = calculate(&monkeys, root_l, &mut cache);
    let f_root_r = calculate(&monkeys, root_r, &mut cache);
    assert_ne!(f_root_l.is_some(), f_root_r.is_some());

    let human_value = if f_root_l.is_none() {
        determine_human(&monkeys, root_l, &mut cache, f_root_r.unwrap())
    } else {
        determine_human(&monkeys, root_r, &mut cache, f_root_l.unwrap())
    };

    println!("[hard] Human should say: {}", human_value);
}

fn calculate(monkeys: &BTreeMap<String, Monkey>,
             name: &str,
             cache: &mut BTreeMap<String, Option<i64>>) -> Option<i64> {
    if cache.contains_key(name) {
        return *cache.get(name).unwrap();
    }

    let monkey = monkeys.get(name).unwrap();
    let res = match monkey {
        Number(x) => Some(*x),
        Undefined => None,
        Operator(name1, name2, op) => {
            let res1 = calculate(monkeys, &name1[..], cache);
            let res2 = calculate(monkeys, &name2[..], cache);

            let result = op.eval(res1, res2);

            result
        }
    };

    cache.insert(String::from(name), res);
    return res;
}

fn determine_human(monkeys: &BTreeMap<String, Monkey>,
                   node: &str,
                   cache: &mut BTreeMap<String, Option<i64>>,
                   target_value: i64) -> i64 {
    let monkey = monkeys.get(node).unwrap();
    assert!(cache.contains_key(node));

    match monkey {
        Undefined => return target_value,
        Number(_) => panic!("Somehow we ended up in non-human branch"),
        Operator(name1, name2, op) => {
            let f_left = *cache.get(name1).unwrap();
            let f_right = *cache.get(name2).unwrap();

            assert_ne!(f_left.is_some(), f_right.is_some());

            let new_target = op.recover_operand(f_left, f_right, target_value);
            let new_node = if f_left.is_none() {
                &name1[..]
            } else {
                &name2[..]
            };

            return determine_human(monkeys, new_node, cache, new_target);
        }
    }
}
