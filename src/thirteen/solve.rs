use std::{cmp::Ordering, iter::Peekable};

use crate::util::read::read;

pub(crate) fn a(input: &str) -> usize {
    let binding = read(input, |l| -> String { l.unwrap() }).collect::<Vec<_>>();
    binding
        .split(|l| l.is_empty())
        .map(|c| {
            (
                parse(&mut c[0].chars().peekable()).remove(0),
                parse(&mut c[1].chars().peekable()).remove(0),
            )
        })
        .map(|(lhs, rhs)| compare(&lhs, &rhs))
        .enumerate()
        .fold(0, |acc, (i, res)| match res {
            Ordering::Less => acc + i + 1,
            _ => acc,
        })
}

pub(crate) fn b(input: &str) -> usize {
    let d1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let d2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    let packets: Vec<Packet> = read(input, |l| -> String { l.unwrap() })
        .filter(|l| !l.is_empty())
        .map(|l| parse(&mut l.chars().peekable()).remove(0))
        .collect();
    let mut t = vec![d1.clone(), d2.clone()];
    t.extend(packets);
    t.sort_by(compare);

    let i1 = t.binary_search_by(|p| compare(p, &d1)).unwrap() + 1;
    let i2 = t.binary_search_by(|p| compare(p, &d2)).unwrap() + 1;

    i1 * i2
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

fn parse(packet: &mut Peekable<impl Iterator<Item = char>>) -> Vec<Packet> {
    let mut elements = vec![];
    while let Some(c) = packet.next() {
        match c {
            ',' => {}
            '[' => elements.push(Packet::List(parse(packet))), // create new child
            ']' => {
                return elements;
            } // go to parent
            x => {
                let mut temp = x.to_string();
                while let Some(test) = packet.peek() {
                    if !test.is_ascii_digit() {
                        break;
                    }
                    let x = packet.next().unwrap();
                    temp.push(x);
                }
                elements.push(Packet::Int(temp.parse().unwrap())) // append x
            }
        }
    }
    elements
}

fn compare(this: &Packet, other: &Packet) -> Ordering {
    match (this, other) {
        (&Packet::Int(lhs), &Packet::Int(rhs)) => lhs.cmp(&rhs),
        (&Packet::List(ref lhs), &Packet::List(ref rhs)) => {
            let mut liter = lhs.iter();
            let mut riter = rhs.iter();

            loop {
                let left = liter.next();
                let right = riter.next();

                match (left, right) {
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (None, None) => return Ordering::Equal,
                    (Some(left), Some(right)) => match compare(left, right) {
                        Ordering::Equal => continue,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                    },
                }
            }
        }
        (&Packet::Int(lhs), rhs) => compare(&Packet::List(vec![Packet::Int(lhs)]), rhs),
        (lhs, &Packet::Int(rhs)) => compare(lhs, &Packet::List(vec![Packet::Int(rhs)])),
    }
}
