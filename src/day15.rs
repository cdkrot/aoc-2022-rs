use std::collections::BTreeSet;

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)){
    let (lhs, rhs) = line.split_once(":").unwrap();

    let lhs = lhs.split_once("at").unwrap().1;
    let rhs = rhs.split_once("at").unwrap().1;

    let (x1, y1) = lhs.split_once(",").unwrap();
    let (x2, y2) = rhs.split_once(",").unwrap();

    let remove_eq = |s: &str| s.split_once("=").unwrap().1.parse().unwrap();

    return ((remove_eq(x1), remove_eq(y1)), (remove_eq(x2), remove_eq(y2)));
}

fn read_input() -> Vec<((i32, i32), (i32, i32))>{
    return std::io::stdin()
        .lines()
        .map(|x| parse_line(&x.unwrap()[..]))
        .collect();
}

pub(crate) fn main() {
    let input = read_input();

    easy(input.clone(), 2000000);
    hard(input.clone(), 0, 4000000);
}

fn get_events_for_y(input: &Vec<((i32, i32), (i32, i32))>, the_y: i32) -> Vec<(i32, bool)>{
    let mut events = vec!();
    for ((x1, y1), (x2, y2)) in input {
        // covering points within dist
        let dist = (x1 - x2).abs() + (y1 - y2).abs();

        let delta = dist - (y1 - the_y).abs();
        if delta < 0 {
            continue;
        }
        let low_x = x1 - delta;
        let hi_x = x1 + delta;

        events.push((low_x, true));
        events.push((hi_x + 1, false));
    }

    events.sort();
    return events;
}

fn easy(input: Vec<((i32, i32), (i32, i32))>, the_y: i32) {
    let mut known_beacons = BTreeSet::new();

    for (_, (x, y)) in &input {
        if *y == the_y {
            known_beacons.insert(*x);
        }
    }

    let events = get_events_for_y(&input, the_y);
    let mut ans = 0;
    let mut prev_x = i32::MIN;
    let mut num_open = 0;
    for (x, is_open) in events {
        if num_open > 0 {
            ans += x - prev_x;
        }

        if is_open {
            num_open += 1;
        } else {
            num_open -= 1;
        }
        prev_x = x;
    }

    println!("Ans: {}", ans - (known_beacons.len() as i32));
}

fn hard(input: Vec<((i32, i32), (i32, i32))>, minc: i32, maxc: i32) {
    for y in minc..maxc+1 {
        let mut events = get_events_for_y(&input, y);
        // some non-real events so that whole [minc, maxc] is covered by the scan line.
        events.push((minc - 2, true));
        events.push((minc - 1, false));
        events.push((maxc + 1, true));
        events.push((maxc + 2, false));
        events.sort();

        let mut prev_x = i32::MIN;
        let mut num_open = 0;
        for (x, is_open) in events {
            if num_open == 0 && prev_x.max(minc) <= (x - 1).min(maxc) {
                let resx = prev_x.max(minc);
                println!("Found: {} {}", resx, y);
                println!("Score: {}", (resx as i64) * (maxc as i64) + (y as i64));
                return;
            }

            if is_open {
                num_open += 1;
            } else {
                num_open -= 1;
            }
            prev_x = x;
        }
    }

    panic!();
}