use std::collections::HashSet;
use crate::utils;

fn distinct(s: &str) -> bool {
    let s_set = s.chars().collect::<HashSet<char>>();

    return s_set.len() == s.len();
}

pub(crate) fn main() {
    let line = utils::read_line();

    let mut pos = 3;
    while !distinct(&line[pos-3..pos+1]) {
        pos += 1;
    }

    let mut pos14 = 13;
    while !distinct(&line[pos14-13..pos14+1]) {
        pos14 += 1;
    }


    println!("Position: {}", pos+1);
    println!("Position: {}", pos14 + 1);
}

