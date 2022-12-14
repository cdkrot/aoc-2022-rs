fn parse_paths() -> Vec<Vec<(usize, usize)>> {
    return std::io::stdin().lines().map(
        |line| line.unwrap().split("->")
            .map(|x| x.trim().split_once(",").unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect()
    ).collect();
}

pub(crate) fn main() {
    let paths = parse_paths();

    solve(paths.clone(), None);
    let floor = paths.iter()
        .map(|path| path.iter().map(|p| p.1).max().unwrap())
        .max().unwrap() + 2;
    solve(paths, Some(floor));
}

fn solve(paths: Vec<Vec<(usize, usize)>>, floor: Option<usize>) {
    let width = 1000;
    let height = floor.unwrap_or(500) + 1;

    let mut matrix = vec![vec!['.'; width]; height];
    if floor.is_some() {
        for i in 0..width {
            matrix[height - 1][i] = '#';
        }
    }

    for path in paths {
        let mut prev = path[0];
        for &cur in &path[1..] {
            let min = (prev.0.min(cur.0), prev.1.min(cur.1));
            let max = (prev.0.max(cur.0), prev.1.max(cur.1));
            for (x, y) in itertools::iproduct!(min.0 .. max.0 + 1, min.1 .. max.1 + 1) {
                matrix[y][x] = '#';
            }
            prev = cur;
        }
    }

    let mut count = 0;
    loop {
        let mut pos = (0, 500);
        if matrix[pos.0][pos.1] != '.' {
            break;
        }

        let mut landed = false;
        while pos.0 != height - 1 {
            if matrix[pos.0 + 1][pos.1] == '.' {
                pos.0 += 1;
            } else if pos.1 > 0 && matrix[pos.0 + 1][pos.1 - 1] == '.' {
                pos.0 += 1;
                pos.1 -= 1;
            } else if pos.1 + 1 < width && matrix[pos.0 + 1][pos.1 + 1] == '.' {
                pos.0 += 1;
                pos.1 += 1;
            } else {
                matrix[pos.0][pos.1] = '#';
                landed = true;
                break;
            }
        }

        if landed {
            count += 1;
        } else {
            break;
        }
    }

    println!("Blocks: {}", count);
}