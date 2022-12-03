extern crate core;

use crate::utils;


pub(crate) fn main() {
    let mut cur_person = 0;
    let mut list_of_people: Vec<i32> = vec![];

    loop {
        match utils::maybe_read_line() {
            Some(line) => match line.trim() {
                "" => {
                    list_of_people.push(cur_person);
                    cur_person = 0;
                },
                x => {
                    cur_person += x.parse::<i32>().unwrap();
                }
            },
            None => {
                list_of_people.push(cur_person);
                break
            }
        }
    }

    list_of_people.sort();
    list_of_people.reverse();
    println!("Heaviest: {}", list_of_people[0]);
    println!("Top3: {}", list_of_people[0..3].iter().sum::<i32>())
}
