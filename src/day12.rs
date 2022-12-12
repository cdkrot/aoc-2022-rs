use std::cmp::min;
use std::collections::VecDeque;
use crate::utils;

fn get_elevation(c: char) -> i32 {
    match c {
        'S' => 'a' as i32,
        'E' => 'z' as i32,
        c => c as i32
    }
}

fn find(map: &Vec<&[u8]>, n: usize, m: usize, c: char) -> (usize, usize) {
    for (i, j) in itertools::iproduct!(0..n, 0..m) {
        if map[i][j] as char == c {
            return (i, j);
        }
    }

    panic!();
}

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub(crate) fn main() {
    let lines = utils::read_all_lines();
    let map: Vec<&[u8]> = lines.iter().map(|s| s.trim().as_bytes()).collect();

    let n = map.len();
    let m = map[0].len();

    let mut dist = vec![vec![-1; m]; n];

    let s = find(&map, n, m, 'S');
    let t = find(&map, n, m, 'E');
    dist[t.0][t.1] = 0;

    let mut q = VecDeque::new();
    q.push_back(t);

    while !q.is_empty() {
        let v_ = q.pop_front().unwrap();
        let v = (v_.0 as i32, v_.1 as i32);

        for (dx, dy) in DIRS {
            if 0 <= v.0 + dx && v.0 + dx < n as i32 &&
                0 <= v.1 + dy && v.1 + dy < m as i32 {
                let u = ((v.0 + dx) as usize, (v.1 + dy) as usize);

                if get_elevation(map[u.0][u.1] as char) >=
                    get_elevation(map[v_.0][v_.1] as char) - 1 && dist[u.0][u.1] == -1 {
                    dist[u.0][u.1] = dist[v_.0][v_.1] + 1;
                    q.push_back(u);
                }
            }
        }
    }

    println!("Distance: {}", dist[s.0][s.1]);

    let mut smallest_distance = dist[s.0][s.1];
    for (i, j) in itertools::iproduct!(0..n, 0..m) {
        if get_elevation(map[i][j] as char) == get_elevation('a') &&
            dist[i][j] != -1 {
            smallest_distance = min(smallest_distance, dist[i][j]);
        }
    }

    println!("Smallest distance: {}", smallest_distance);
}