use std::cmp::Ordering;

use crate::util::read::read;

pub(crate) fn a(input: &str) -> usize {
    let binding = read(input, |l| -> String { l.unwrap() }).collect::<Vec<_>>();
    binding
        .split(|l| l.is_empty())
        .map(|c| {
            (
                parse(&mut c[0].chars()).remove(0),
                parse(&mut c[1].chars()).remove(0),
            )
        })
        .map(|(lhs, rhs)| compare(&lhs, &rhs))
        .enumerate()
        .fold(0, |acc, (i, res)| match res {
            Ordering::Less => acc + i + 1,
            _ => acc,
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

fn parse(packet: &mut impl Iterator<Item = char>) -> Vec<Packet> {
    let mut elements = vec![];
    while let Some(c) = packet.next() {
        match c {
            ',' => {}
            '[' => elements.push(Packet::List(parse(packet))), // create new child
            ']' => {
                return elements;
            } // go to parent
            x => elements.push(Packet::Int(x.to_digit(10).unwrap())), // append x
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
