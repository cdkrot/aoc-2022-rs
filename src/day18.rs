use std::collections::{BTreeSet, VecDeque};

const DIRS: [(i32, i32, i32); 6] = [
    (0, 0, 1), (0, 0, -1),
    (0, 1, 0), (0, -1, 0),
    (1, 0, 0), (-1, 0, 0),
];

fn sum(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    return (a.0 + b.0, a.1 + b.1, a.2 + b.2);
}

pub(crate) fn main() {
    let input: Vec<Vec<i32>> = std::io::stdin()
        .lines()
        .map(|s| s.unwrap())
        .map(|s| s.split(",").map(|tok| tok.parse().unwrap()).collect())
        .collect();

    let input: Vec<(i32, i32, i32)> = input.iter().map(
        |coords| (coords[0], coords[1], coords[2])
    ).collect();

    easy(input.clone());
    hard(input);
}

fn easy(input: Vec<(i32, i32, i32)>) {
    let input_set: BTreeSet<(i32, i32, i32)> = input.iter().map(|v| *v).collect();
    let mut ans = 6 * input.len();

    for coords in input {
        for dir in DIRS {
            if input_set.get(&sum(coords, dir)).is_some() {
                ans -= 1;
            }
        }
    }

    println!("ans: {}", ans);
}

fn hard(input: Vec<(i32, i32, i32)>) {
    let input_set: BTreeSet<(i32, i32, i32)> = input.iter().map(|v| *v).collect();

    let is_open_face = |(coords, dir): ((i32, i32, i32), usize)| {
        let dir = DIRS[dir];
        let adj = sum(coords, dir);

        assert!(input_set.get(&coords).is_some());

        return !(input_set.get(&adj).is_some());
    };

    // left-facing face.
    let start = (*input.iter().min().unwrap(), 5);
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::new();
    visited.insert(start);
    queue.push_back(start);

    while !queue.is_empty() {
        let (v, dirv) = queue.pop_front().unwrap();

        for other_dir in 0..6 {
            if (dirv ^ 1) == other_dir {
                continue; // can't jump to opposite face
            }

            if input_set.contains(&sum(sum(v, DIRS[dirv]), DIRS[other_dir])) {
                continue; // blocked, can't claim to be open face.
            }

            if is_open_face((v, other_dir)) && !visited.contains(&(v, other_dir)) {
                visited.insert((v, other_dir));
                queue.push_back((v, other_dir));
            }
        }

        // adjacent face with same direction
        for dir in 0..6 {
            if dir == dirv || dir == (dirv ^ 1) {
                continue;
            }

            let u = sum(v, DIRS[dir]);
            if input_set.contains(&u) &&
                is_open_face((u, dirv)) &&
                !visited.contains(&(u, dirv)) {

                visited.insert((u, dirv));
                queue.push_back((u, dirv));
            }
        }

        // other vertices faces.
        let vprime = sum(v, DIRS[dirv]);
        for dir in 0..6 {
            if dir == dirv || dir == (dirv ^ 1) {
                continue; // can't go in the same direction, or reverse direction.
            }

            let u = sum(vprime, DIRS[dir]);
            if input_set.contains(&u) &&
                is_open_face((u, dir ^ 1)) &&
                !visited.contains(&(u, dir ^ 1)) {

                visited.insert((u, dir^1));
                queue.push_back((u, dir^1));
            }
        }
    }

    println!("outer face: {}", visited.len());
}
