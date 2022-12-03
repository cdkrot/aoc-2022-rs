use std::process::abort;
use crate::utils;

fn map_xyz_to_abc(a: &str) -> &str {
    match a {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => abort()
    }
}

fn xyz_to_match_result(a: &str) -> i32 {
    match a {
        "X" => -1,
        "Y" => 0,
        "Z" => 1,
        _ => abort()
    }
}

fn match_result(a: &str, b: &str) -> i32 {
    match (a, b) {
        ("A", "A") => 0,
        ("B", "B") => 0,
        ("C", "C") => 0,
        ("A", "B") => 1,
        ("B", "C") => 1,
        ("C", "A") => 1,
        (_, _) => -1
    }
}

fn get_score(npc: &str, our_move: &str) -> i32 {
    let mut score = 3 + 3 * match_result(npc, our_move);
    score += match our_move {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => abort()
    };
    return score;
}

pub(crate) fn main() {
    let mut score = 0;
    let mut score_pt2 = 0;
    loop {
        match utils::maybe_read_line() {
            Some(line) => {
                let pieces: Vec<&str> = line.trim().split(" ").collect();

                let npc = pieces[0];
                let our_move = map_xyz_to_abc(pieces[1]);
                let match_result_pt2 = xyz_to_match_result(pieces[1]);

                let our_move_pt2 = vec!("A", "B", "C").into_iter().filter(
                    |mv| match_result(npc, mv) == match_result_pt2).nth(0).unwrap();

                score += get_score(npc, our_move);
                score_pt2 += get_score(npc, our_move_pt2);
            },
            None => break
        }
    }

    println!("Score: {}", score);
    println!("Score2: {}", score_pt2);

}