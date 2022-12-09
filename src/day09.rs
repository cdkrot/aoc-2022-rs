use std::collections::BTreeSet;
use crate::utils;

pub(crate) fn main() {
    let lines = utils::read_all_lines();

    rope_simulation(lines.clone(), 2);
    rope_simulation(lines, 10);
}

fn rope_simulation(lines: Vec<String>, num_knots: usize) {
    let mut knots = vec!();
    for _ in 0..num_knots {
        knots.push((0, 0));
    }

    let mut visited: BTreeSet<(i32, i32)> = BTreeSet::new();
    visited.insert(*knots.last().unwrap());

    for line in lines {
        let (dir, cnt) = line.trim().split_once(' ').unwrap();

        for _ in 0..cnt.parse().unwrap() {
            match dir {
                "L" => knots[0].0 -= 1,
                "R" => knots[0].0 += 1,
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                _ => panic!("Unknown direction"),
            }

            for k in 1..num_knots {
                if (knots[k - 1].0 - knots[k].0).abs() >= 2 ||
                    (knots[k - 1].1 - knots[k].1).abs() >= 2 {

                    if knots[k].0 < knots[k - 1].0 {
                        knots[k].0 += 1;
                    } else if knots[k].0 > knots[k - 1].0 {
                        knots[k].0 -= 1;
                    }

                    if knots[k].1 < knots[k - 1].1 {
                        knots[k].1 += 1;
                    } else if knots[k].1 > knots[k - 1].1 {
                        knots[k].1 -= 1;
                    }
                }
            }

            visited.insert(*knots.last().unwrap());
        }
    }

    println!("Total visited: {}", visited.len());
}
