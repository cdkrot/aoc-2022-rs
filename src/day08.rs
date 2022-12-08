use std::cmp::max;
use crate::utils::read_all_lines;

pub(crate) fn main() {
    let lines: Vec<Vec<char>> = read_all_lines().iter().map(|s| s.trim().chars().collect()).collect();

    let mut num_visible = 0;
    let mut max_view_dist = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if is_visible(&lines, i as i32, j as i32) {
                num_visible += 1;
            }

            max_view_dist = max(max_view_dist, fourway_view_distance(&lines, i as i32, j as i32));
        }
    }

    println!("Visible: {}", num_visible);
    println!("Max 4-way view dist: {}", max_view_dist);
}

fn is_visible(mat: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    for dir in 0..4 {
        if viewing_distance(mat, i, j, dir).1 {
            return true;
        }
    }
    return false;
}

static DIRX: [i32; 4] = [0, 0, -1, 1];
static DIRY: [i32; 4] = [-1, 1, 0, 0];

fn viewing_distance(mat: &Vec<Vec<char>>, i: i32, j: i32, dir: usize) -> (i32, bool, i32, i32) {
    // view from a given cell in direction of dir
    // return number of steps until terminated
    // if all viewed trees are shorter than us
    // the final (i, j) coordinates (possibly outside the board).

    let n: i32 = mat.len() as i32;
    let m: i32 = mat[0].len() as i32;

    let mut steps = 0;

    let mut curi = i;
    let mut curj = j;

    let mut all_shorter = true;

    loop {
        curi += DIRX[dir];
        curj += DIRY[dir];
        steps += 1;

        if !(curi >= 0 && curi < n && curj >= 0 && curj < m) {
            break;
        }

        all_shorter = mat[curi as usize][curj as usize] < mat[i as usize][j as usize];
        if !all_shorter {
            break;
        }
    }

    return (steps, all_shorter, curi, curj);
}

fn fourway_view_distance(mat: &Vec<Vec<char>>, i: i32, j: i32) -> i64 {
    let mut result: i64 = 1;
    let n: i32 = mat.len() as i32;
    let m: i32 = mat[0].len() as i32;

    for dir in 0..4 {
        let (mut steps, _, fini, finj) = viewing_distance(mat, i, j, dir);

        if fini < 0 || finj < 0 || fini >= n || finj >= m {
            steps -= 1;
        }
        result *= steps as i64;
    }
    return result;
}
