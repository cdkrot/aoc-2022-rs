use crate::utils;

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    func: Function,
    test_divisor: i64,
    pass_true: usize,
    pass_false: usize,
}

#[derive(Clone, Copy)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Clone, Copy)]
struct Function {
    // None represents "old"
    lhs: Option<i64>,
    rhs: Option<i64>,
    op: Operation,
}

impl Function {
    fn eval(&self, old: i64) -> i64 {
        let lhs = self.lhs.unwrap_or(old);
        let rhs = self.rhs.unwrap_or(old);

        match self.op {
            Operation::Multiply => lhs * rhs,
            Operation::Add => lhs + rhs
        }
    }
}

fn parse_operand(op: &str) -> Option<i64> {
    match op {
        "old" => None,
        num => Some(num.parse().unwrap())
    }
}

pub(crate) fn main() {
    let monkeys = read_monkeys();

    let mut modulo: i64 = 1;
    for monkey in &monkeys {
        modulo = modulo * monkey.test_divisor;
    }

    simul_monkeys(monkeys.clone(), 0, true, 20);
    simul_monkeys(monkeys.clone(), modulo, false, 10000);
}

fn simul_monkeys(mut monkeys: Vec<Monkey>, modulo: i64, divby3: bool, rounds: usize) {
    let mut stats = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            monkeys[i].items.clear();
            stats[i] += monkey.items.len();

            for item in monkey.items {
                let mut item = monkey.func.eval(item);
                if divby3 {
                    item = item / 3;
                }
                if modulo != 0 {
                    item = item % modulo;
                }

                let pass_to = if item % monkey.test_divisor == 0 {
                    monkey.pass_true
                } else {
                    monkey.pass_false
                };

                monkeys[pass_to].items.push(item);
            }
        }
    }

    stats.sort();
    stats.reverse();
    println!("'Monkey business: {}", (stats[0] as u64) * (stats[1] as u64));
}

fn read_monkeys() -> Vec<Monkey> {
    let mut monkeys = vec!();

    while utils::maybe_read_line().is_some() {
        let items: Vec<i64> = utils::read_line()
            .split_once(':')
            .unwrap().1
            .split(',')
            .map(|tok| tok.trim().parse::<i64>().unwrap())
            .collect();
        let f_desc_line = utils::read_line();
        let f_desc: Vec<&str> = f_desc_line
            .split_once('=')
            .unwrap().1
            .trim()
            .split(' ')
            .collect();
        let func = Function {
            lhs: parse_operand(f_desc[0]),
            rhs: parse_operand(f_desc[2]),
            op: match f_desc[1] {
                "*" => Operation::Multiply,
                "+" => Operation::Add,
                _ => panic!()
            }
        };

        let test_divisor = utils::read_line()
            .split_once("by")
            .unwrap().1
            .trim()
            .parse::<i64>()
            .unwrap();

        let pass_true = utils::read_line()
            .split_once("monkey").unwrap().1.trim()
            .parse::<usize>().unwrap();
        let pass_false = utils::read_line()
            .split_once("monkey").unwrap().1.trim()
            .parse::<usize>().unwrap();

        monkeys.push(Monkey {
            items,
            func,
            test_divisor,
            pass_true,
            pass_false,
        });

        utils::maybe_read_line(); //blank spacing
    }

    return monkeys;
}