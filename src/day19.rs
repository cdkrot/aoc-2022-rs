use std::collections::{BTreeMap, BTreeSet};
use std::io::stdin;
use crate::utils;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}
use Resource::*;

struct Blueprint {
    cost: BTreeMap<Resource, Vec<(Resource, i32)>>,
    id: i32,
}

#[derive(Clone, Debug)]
struct BruteState {
    time_remaining: i32,
    num_robots: BTreeMap<Resource, i32>,
    num_resource: BTreeMap<Resource, i32>,
    freeze: BTreeSet<Resource>
}

impl BruteState {
    fn produce(&mut self) {
        for (resource, quantity) in self.num_robots.iter() {
            *self.num_resource.entry(*resource).or_insert(0) += quantity;
        }
    }
}

fn parse_resource_desc(s: &str) -> (Resource, i32) {
    let (quantity, resource) = s.split_once(' ').unwrap();
    let resource = match resource {
        "ore" => Ore,
        "clay" => Clay,
        "obsidian" => Obsidian,
        "geode" => Geode,
        _ => panic!()
    };

    return (resource, quantity.parse().unwrap());
}

fn parse_blueprint(s: &str) -> Blueprint {
    let s = utils::expect_prefix(s, "Blueprint ");
    let (num, s) = s.split_once(":").unwrap();

    let s = if s.ends_with(".") {
        &s[..s.len()-1]
    } else {
        &s[..]
    };

    let pieces: Vec<Vec<(Resource, i32)>> = s.split(".")
        .map(|x| x.split_once("costs").unwrap().1)
        .map(|x| x.split("and").map(|s| parse_resource_desc(s.trim())).collect())
        .collect();


    return Blueprint {
        cost: BTreeMap::from([
                                 (Ore, pieces[0].clone()),
                                 (Clay, pieces[1].clone()),
                                 (Obsidian, pieces[2].clone()),
                                 (Geode, pieces[3].clone())]),
        id: num.parse().unwrap()
    }
}

pub(crate) fn main() {
    let blueprints: Vec<Blueprint> = stdin()
        .lines()
        .map(|s| parse_blueprint(&s.unwrap()[..]))
        .collect();

    easy(&blueprints[..]);
    hard(&blueprints[..]);
}

fn easy(blueprints: &[Blueprint]) {
    let ans: i32 = blueprints.iter().map(|b|
        blueprint_quality(24,&b) * b.id).sum();

    println!("Ans: {}", ans);
}

fn hard(blueprints: &[Blueprint]) {
    let blueprints = &blueprints[..3];

    let nums: Vec<i32> = blueprints.iter().map(|b|
    blueprint_quality(32, &b)).collect();

    println!("ans: {}", nums.iter().map(|x| *x as i64).product::<i64>());
}


fn blueprint_quality(time: i32, blueprint: &Blueprint) -> i32 {
    println!("Processing blueprint {}", blueprint.id);

    return brute(&blueprint, &mut BruteState {
        time_remaining: time,
        num_robots: BTreeMap::from([(Ore, 1)]),
        num_resource: Default::default(),
        freeze: Default::default(),
    });
}

fn brute(blueprint: &Blueprint, state: &mut BruteState) -> i32 {
    let mut ans: i32 = state.num_resource.get(&Geode).unwrap_or(&0)
        + state.time_remaining * state.num_robots.get(&Geode).unwrap_or(&0);

    if state.time_remaining == 0 {
        return ans;
    }


    // we don't want to build resource machines, if we have enough income to build everything needed
    // that doesn't apply to Geode though.
    for resource in [Ore, Clay, Obsidian] {
        let maxcost = blueprint.cost.values()
            .flatten()
            .filter(|&&x| x.0 == resource)
            .map(|x| x.1).max().unwrap();
        if *state.num_robots.get(&resource).unwrap_or(&0) >= maxcost {
            state.freeze.insert(resource);
        }
    }

    //println!("{:?} {}", state, ans);

    for (resource, cost) in blueprint.cost.iter() {
        if cost.iter().all(|(r, need)|
            state.num_resource.get(r).unwrap_or(&0) >= need) {

            if state.freeze.get(resource).is_some() {
                continue;
            }

            let mut new_state: BruteState = state.clone();
            new_state.time_remaining -= 1;

            for (r, need) in cost.iter() {
                *new_state.num_resource.entry(*r).or_insert(0) -= need;
            }
            new_state.freeze.clear();
            new_state.produce();
            *new_state.num_robots.entry(*resource).or_insert(0) += 1;
            ans = ans.max(brute(blueprint, &mut new_state));
        }
    }

    // we decided not to produce -- add freezes.
    for (resource, cost) in blueprint.cost.iter() {
        if cost.iter().all(|(r, need)|
            state.num_resource.get(r).unwrap_or(&0) >= need) {

            state.freeze.insert(*resource);
        }
    }

    state.time_remaining -= 1;
    state.produce();

    let tmp = state;
    ans = ans.max(brute(blueprint, tmp));
    return ans;
}
