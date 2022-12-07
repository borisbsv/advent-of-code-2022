use crate::util::read::read;
use regex::Regex;

pub(crate) fn a(input: &str) -> String {
    let lines: Vec<_> = read(input, |l| l.unwrap().chars().collect::<Vec<char>>()).collect();
    let l = lines.split(|l| l.is_empty()).collect::<Vec<_>>();

    let mut stacks = to_stacks(l.first().unwrap().iter().rev().skip(1).collect());
    let actions = to_actions(l.last().unwrap().to_vec());

    for (amount, from, to) in actions {
        for _ in 0..amount {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    stacks
        .into_iter()
        .map(|mut s| s.pop().unwrap_or('\0'))
        .collect()
}

pub(crate) fn b(input: &str) -> String {
    let lines: Vec<_> = read(input, |l| l.unwrap().chars().collect::<Vec<char>>()).collect();
    let l = lines.split(|l| l.is_empty()).collect::<Vec<_>>();

    let mut stacks = to_stacks(l.first().unwrap().iter().rev().skip(1).collect());
    let actions = to_actions(l.last().unwrap().to_vec());

    for (amount, from, to) in actions {
        let at = stacks[to].len();
        for _ in 0..amount {
            let c = stacks[from].pop().unwrap();
            stacks[to].insert(at, c);
        }
    }

    stacks
        .into_iter()
        .map(|mut s| s.pop().unwrap_or('\0'))
        .collect()
}

fn to_actions(a_lines: Vec<Vec<char>>) -> Vec<(usize, usize, usize)> {
    let command_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut actions = Vec::new();
    for line in a_lines {
        let cmd = command_re
            .captures(&line.into_iter().collect::<String>().to_owned())
            .unwrap()
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        actions.push((cmd[0], cmd[1] - 1, cmd[2] - 1));
    }
    actions
}

fn to_stacks(lines: Vec<&Vec<char>>) -> Vec<Vec<char>> {
    const WIDTH: usize = 4;
    let s_len = (lines[0].len() + 1) / WIDTH;
    let mut stacks = Vec::with_capacity(s_len);
    for _ in 0..s_len {
        stacks.push(Vec::new());
    }

    for l in lines {
        for i in 0..s_len {
            let c = l[1 + i * WIDTH];
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    stacks
}
