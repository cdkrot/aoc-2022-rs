use std::str::FromStr;
use std::io::Read;

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
pub(crate) fn read_token() -> String {
    let mut token = String::new();
    std::io::stdin().read_to_string(&mut token).unwrap();

    return token;
}

#[allow(dead_code)]
pub(crate) fn read_int() -> i32 {
    let line = read_token();

    return line.trim().parse().unwrap();
}

#[allow(dead_code)]
pub(crate) fn maybe_read_int() -> Result<i32, <i32 as FromStr>::Err> {
    let line = read_token();

    return line.trim().parse();
}