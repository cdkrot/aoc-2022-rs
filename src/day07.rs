use std::collections::{BTreeMap, BTreeSet};
use crate::utils;

pub(crate) fn main() {
    let mut tree_size: BTreeMap<Vec<&str>, i64> = BTreeMap::new();
    let mut listed_nodes: BTreeSet<Vec<&str>> = BTreeSet::new();

    let mut path: Vec<&str> = vec!();
    let lines = utils::read_all_lines();
    let mut p_command = 0;
    while p_command < lines.len() {
        let cur_line = &lines[p_command].trim();
        let parts: Vec<&str> = cur_line.split(' ').collect();
        p_command += 1;

        match parts[1] {
            "cd" => {
                match parts[2] {
                    "/" => path = vec!(),
                    ".." => { path.pop(); }
                    x => path.push(x),
                }
            },
            "ls" => {
                let was_listed = listed_nodes.contains(&*path);

                while p_command < lines.len() &&
                    !lines[p_command].starts_with("$") {
                    let parts = lines[p_command].split_once(" ").unwrap();
                    p_command += 1;

                    if !was_listed && parts.0 != "dir" {
                        let file_size = parts.0.parse::<i64>().unwrap();
                        for prefix in 0..path.len() + 1 {
                            let prefix_path: Vec<&str> = path[0..prefix].to_vec();

                            tree_size.entry(prefix_path).and_modify(
                                |sz| *sz += file_size
                            ).or_insert(file_size);
                        }
                    }
                }

                listed_nodes.insert(path.clone());
            },
            _ => {
                panic!("Unrecognized command {}", cur_line);
            }
        }
    }

    let small_directories = tree_size
        .iter()
        .map(|(_vec, sz)| *sz)
        .filter(|sz| *sz <= 100000)
        .sum::<i64>();

    println!("Small directories: {}", small_directories);

    let total_size = *tree_size.entry(vec!()).or_default();
    let size_to_remove = total_size - (70000000 - 30000000);
    assert!(size_to_remove > 0);

    let removal = tree_size
        .iter()
        .map(|(_vec, sz)| *sz)
        .filter(|sz| *sz >= size_to_remove)
        .min()
        .unwrap();

    println!("Removed directory size: {}", removal);
}