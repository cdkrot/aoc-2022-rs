use std::fmt::Debug;
use std::str::FromStr;

#[allow(dead_code)]
pub(crate) fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    return line;
}

#[allow(dead_code)]
pub(crate) fn maybe_read_line() -> Option<String> {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => Some(line),
        Err(_) => None
    }
}

#[allow(dead_code)]
pub(crate) fn read_int<Num : FromStr>() -> Num where <Num as FromStr>::Err: Debug {
    let line = read_line();

    return line.trim().parse().unwrap();
}

pub(crate) fn read_all_lines() -> Vec<String> {
    let mut lines = vec!();

    while let Some(line) = maybe_read_line() {
        lines.push(line);
    }

    return lines;
}

#[allow(dead_code)]
pub(crate) fn expect_prefix<'a>(s: &'a str, expected: &str) -> &'a str {
    if !s.starts_with(expected) {
        panic!();
    }

    return &s[expected.len()..];
}