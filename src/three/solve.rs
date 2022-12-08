use std::collections::HashSet;

use crate::util::read::read;

pub(crate) fn a(input: &str) -> i32 {
    let lines = read(input, |l| l.unwrap().as_bytes().to_owned());
    lines
        .map(|l| {
            let (lhs, rhs) = l.split_at(l.len() / 2);
            for lv in lhs.iter() {
                if rhs.contains(lv) {
                    return priority(*lv);
                }
            }
            0
        })
        .sum::<i32>()
}

pub(crate) fn b(input: &str) -> i32 {
    let lines: Vec<HashSet<u8>> = read(input, |l| {
        l.unwrap()
            .as_bytes()
            .to_owned()
            .iter()
            .cloned()
            .collect::<HashSet<u8>>()
    })
    .collect();
    let mut sum = 0;

    for ch in lines.chunks(3) {
        let mut l1: HashSet<u8> = ch[0].clone();
        l1.retain(|l| ch[1].contains(l) && ch[2].contains(l));

        sum += priority(*l1.iter().next().unwrap());
    }

    sum
}

fn priority(i: u8) -> i32 {
    if i.is_ascii_lowercase() {
        i as i32 - 96
    } else {
        i as i32 - 38
    }
}
