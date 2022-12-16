use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn parse_line(line: &str) -> (String, i32, Vec<String>) {
    if &line[0..6] != "Valve " {
        panic!("Unknown format");
    }

    let line = &line[6..];
    let valve = String::from(&line[0..2]);
    let line = &line[2..];

    let next_wording = " has flow rate=";
    if &line[0..next_wording.len()] != next_wording {
        panic!("Unknown format");
    }

    let line = &line[next_wording.len()..];
    let (rate_str, line) = line.split_once(";").unwrap();
    let rate = rate_str.parse::<i32>().unwrap();

    let next_wording = " tunnels lead to valves ";
    let line = if &line[0..next_wording.len()] == next_wording {
        &line[next_wording.len()..]
    } else {
        let next_wording = " tunnel leads to valve ";

        if &line[0..next_wording.len()] != next_wording {
            panic!("Unknown format");
        }

        &line[next_wording.len()..]
    };

    let directions: Vec<String> = line.split(", ").map(String::from).collect();

    return (valve, rate, directions);
}

fn read_input() -> Vec<(String, i32, Vec<String>)>{
    return std::io::stdin()
        .lines()
        .map(|x| parse_line(&x.unwrap()[..]))
        .collect();
}

pub(crate) fn main() {
    let input = read_input();

    easy(input);
}

fn easy(input: Vec<(String, i32, Vec<String>)>) {
    let input_dict: BTreeMap<String, (i32, Vec<String>)> = input.clone().into_iter()
        .map(|p| (p.0, (p.1, p.2)))
        .collect();

    let interesting: Vec<&str> = input.iter()
        .filter(|(_, rate, _)| *rate > 0)
        .map(|p| &p.0[..])
        .collect();

    fn brute<'a>(vertex: &str, production: i32, remaining_time: i32,
             visited: &mut BTreeSet<&'a str>, input_dict: &BTreeMap<String, (i32, Vec<String>)>,
             interesting: &Vec<&'a str>) -> i32 {
        let distances = calc_distance(input_dict, vertex);

        let mut result = 0;

        for goto in interesting {
            if visited.get(goto).is_some() {
                continue;
            }

            let d = *distances.get(goto).unwrap();
            if d > remaining_time + 1 {
                continue
            }

            visited.insert(*goto);
            let node_rate = input_dict.get(*goto).unwrap().0;
            result = result.max(
                brute(goto, production + node_rate,
                remaining_time - d - 1, visited, input_dict, interesting
                ) + node_rate * (remaining_time - d - 1)
            );
            visited.remove(goto);
        }

        return result;
    }

    let mut visited: BTreeSet<&str> = BTreeSet::new();
    println!("Ans: {}", brute("AA", 0, 30,
                              &mut visited, &input_dict, &interesting));
}

fn calc_distance<'a>(input: &'a BTreeMap<String, (i32, Vec<String>)>, s: &'a str) -> BTreeMap<&'a str, i32> {
    let mut result = BTreeMap::new();
    let mut queue = VecDeque::new();

    result.insert(s, 0);
    queue.push_back(s);

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        let distv = *result.get(v).unwrap();

        for u in input.get(v).unwrap().1.iter() {
            let u = &u[..];

            if result.get(u).is_none() {
                result.insert(u, distv + 1);
                queue.push_back(u);
            }
        }
    }

    return result;
}
