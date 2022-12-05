use itertools::Itertools;

fn all_chars() -> Vec<(i32, char)> {
    let mut result = vec!();

    for ch in 'a'..'{' { // a-z inclusive
        result.push(((ch as i32) - ('a' as i32) + 1, ch));
    }

    for ch in 'A'..'[' { // a-z inclusive
        result.push(((ch as i32) - ('A' as i32) + 27, ch));
    }

    return result;
}

fn cost(a: &str, b: &str) -> i32 {
    for (code, ch) in all_chars() {
        if a.find(ch).is_some() && b.find(ch).is_some() {
            return code;
        }
    }

    panic!("Fail: {} {}", a, b);
}

fn cost_partb(lst: Vec<String>) -> i32 {
    for (code, ch) in all_chars() {
        if lst.iter().all(|str| str.contains(ch)) {
            return code;
        }
    }

    panic!();
}

pub(crate) fn main() {
    let lines: Vec<String> = std::io::stdin().lines().into_iter().map(
        |x| x.unwrap()
    ).collect();
    let result: i32 = lines.iter().map(
        |line| {
            cost(&line[0..line.len() / 2], &line[line.len()/2..line.len()])
        }
    ).sum();

    let result_partb: i32 = lines
        .into_iter()
        .enumerate()
        .group_by(|(a, _)| a / 3)
        .into_iter()
        .map(|(_, lst)| lst.map(|(_, b)| b).collect())
        .map(cost_partb)
        .sum();

    println!("Cost is {}", result);
    println!("Cost2 is {}", result_partb);
}
