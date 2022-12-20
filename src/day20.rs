use std::io::stdin;
use std::ops::{Add, Rem};

pub(crate) fn main() {
    let input: Vec<i32> = stdin()
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    easy(input.clone());
    hard(input);
}

trait ToInt32 {
    fn as_i32(&self) -> i32;
}

impl ToInt32 for i32 {
    fn as_i32(&self) -> i32 {
        *self
    }
}

impl ToInt32 for i64 {
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}

fn mix<T: Add<T, Output=T> + Copy + Eq + Rem<T, Output=T>>(input: Vec<T>, times: usize) -> Vec<T>
    where T: From<i32>, T: ToInt32 {
    let mut input: Vec<(usize, T)> = input.into_iter().enumerate().collect();

    let order = input.clone();

    for _ in 0..times {
        for &val in order.iter() {
            let pos = input.iter().position(|&num| num == val).unwrap() as i32;

            let new_pos = (((T::from(pos)) + val.1) % T::from(input.len() as i32 - 1)).as_i32();
            let new_pos = if new_pos < 0 {
                new_pos + ((input.len() - 1) as i32)
            } else {
                new_pos
            };

            let mut pos = pos as usize;
            let new_pos = new_pos as usize;

            while pos < new_pos {
                let t = input[pos];
                input[pos] = input[pos + 1];
                input[pos + 1] = t;
                pos += 1;
            }

            while pos > new_pos {
                let t = input[pos];
                input[pos] = input[pos - 1];
                input[pos - 1] = t;
                pos -= 1;
            }
        }
    }

    return input.iter().map(|p| p.1).collect();
}

fn easy(input: Vec<i32>) {
    let input = mix(input, 1);

    let pos_zero = input.iter().position(|&num| num == 0).unwrap();
    let result = [1000, 2000, 3000].map(
        |delta| input[(pos_zero + delta) % input.len()]
    );
    println!("{:?}", result);
    println!("ans: {}", result.iter().sum::<i32>());
}

const MAGIC: i64 = 811589153;
fn hard(input: Vec<i32>) {
    let input: Vec<i64> = input.iter().map(|&x| (x as i64) * MAGIC).collect();
    let input = mix(input, 10);

    let pos_zero = input.iter().position(|&num| num == 0).unwrap();
    let result = [1000, 2000, 3000].map(
        |delta| input[(pos_zero + delta) % input.len()]
    );
    println!("{:?}", result);
    println!("ans: {}", result.iter().sum::<i64>());
}