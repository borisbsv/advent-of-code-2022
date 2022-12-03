use std::collections::HashSet;

use crate::util::read::read;

pub fn a(input: &str) -> String {
    let lines = read(input, |l| l.unwrap().as_bytes().to_owned());

    let res: i32 = lines.fold(0, |acc, l| {
        let (lhs, rhs) = l.split_at(l.len() / 2);
        let mut curr = acc;
        for lv in lhs.iter() {
            if rhs.contains(lv) {
                curr += priority(*lv);
                break;
            }
        }
        curr
    });

    res.to_string()
}

pub fn b(input: &str) -> String {
    let lines = read(input, |l| l.unwrap().as_bytes().to_owned()).collect::<Vec<Vec<u8>>>();
    let mut i = 0;
    let mut sum = 0;
    while i < lines.len() {
        let mut l1: HashSet<u8> = lines[i].iter().cloned().collect();
        let l2: HashSet<u8> = lines[i + 1].iter().cloned().collect();
        let l3: HashSet<u8> = lines[i + 2].iter().cloned().collect();
        l1.retain(|l| l2.contains(l) && l3.contains(l));

        sum += priority(*l1.iter().next().unwrap());

        i += 3;
    }

    sum.to_string()
}

fn priority(i: u8) -> i32 {
    if i.is_ascii_lowercase() {
        i as i32 - 96
    } else {
        i as i32 - 38
    }
}
