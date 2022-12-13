use std::cmp::Ordering;
use crate::utils;

#[derive(Eq, PartialEq, Clone)]
enum FList<T> {
    Cons(T, Box<FList<T>>),
    Nil,
}
use FList::{Cons, Nil};

#[derive(Eq, PartialEq, Clone)]
enum Packet {
    List(FList<Box<Packet>>),
    Number(i32)
}
use Packet::{List, Number};

impl<T: Ord> Ord for FList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Nil, Cons(_, _)) => Ordering::Less,
            (Nil, Nil) => Ordering::Equal,
            (Cons(a, b), Cons(c, d)) => {
                match a.cmp(c) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => b.cmp(d),
                    Ordering::Greater => Ordering::Greater
                }
            },
            (a, b) => b.cmp(a).reverse()
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Number(a), Number(b)) => {
                return a.cmp(b)
            },
            (List(a), List(b)) => {
                return a.cmp(b)
            }
            (Number(a), b) => {
                List(Cons(Box::new(Number(*a)), Box::new(Nil))).cmp(b)
            },
            (a, b) => {
                b.cmp(a).reverse()
            }
        }
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> PartialOrd for FList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn _parse_rest_of_the_list(s: &str) -> (FList<Box<Packet>>, &str) {
    match &s[0..1] {
        "]" => {
            (FList::Nil, &s[1..])
        },
        _ => {
            let (packet_head, mut rest) = parse_packet(s);
            if rest.chars().next().unwrap_or(' ') == ',' {
                rest = &rest[1..];
            }

            let (list_rest, rest) = _parse_rest_of_the_list(rest);

            return (Cons(packet_head, Box::new(list_rest)), rest);
        }
    }
}

fn parse_packet(s: &str) -> (Box<Packet>, &str) {
    match &s[0..1] {
        "[" => {
            let (lst, rest) = _parse_rest_of_the_list(&s[1..]);
            return (Box::new(Packet::List(lst)), rest);
        },
        _ => {
            let mut number_str = String::new();
            let mut s = s;
            while s.chars().next().unwrap().is_digit(10) {
                number_str.push(s.chars().next().unwrap());
                s = &s[1..];
            }

            return (Box::new(Packet::Number(number_str.parse().unwrap())),
                    s);
        }
    }
}

pub(crate) fn main() {
    let mut i = 0;
    let mut ans = 0;
    let divider_packet1 = parse_packet("[[2]]").0;
    let divider_packet2 = parse_packet("[[6]]").0;
    let mut all_packets = vec!(divider_packet1.clone(), divider_packet2.clone());

    while let Some(line1) = utils::maybe_read_line() {
        let line2 = utils::read_line();
        utils::maybe_read_line(); // blank line
        i += 1;

        let packet1 = parse_packet(line1.trim()).0;
        let packet2 = parse_packet(line2.trim()).0;
        if packet1 < packet2 {
            ans += i;
        }

        all_packets.push(packet1);
        all_packets.push(packet2);
    }

    println!("ans part 1: {}", ans);

    all_packets.sort();

    let mut answer2 = 1;
    for (i, packet) in all_packets.iter().enumerate() {
        if *packet == divider_packet1 || *packet == divider_packet2 {
            answer2 *= i + 1;
        }
    }

    println!("ans part 2: {}", answer2);
}