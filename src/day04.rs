use std::cmp::{max, min};
use itertools::Itertools;

pub(crate) fn main() {
    let result = std::io::stdin()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {x.split(',')
                        .map(String::from)
                        .collect::<Vec<String>>()})
        .flatten()
        .map(|x| x.split('-')
                        .map(|y| y.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>())
        .flatten()
        .chunks(4);

    let mut answer_nested = 0;
    let mut answer_overlap = 0;

    for chunk in result.into_iter() {
        let nums: Vec<i32> = chunk.collect();

        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        let d = nums[3];

        if (a <= c && d <= b) || (c <= a && b <= d) {
            answer_nested += 1;
        }

        if max(a, c) <= min(b, d) {
            answer_overlap += 1;
        }
    }

    println!("Nested: {}", answer_nested);
    println!("Overlap: {}", answer_overlap);
}