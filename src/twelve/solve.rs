use std::collections::{HashMap, HashSet, VecDeque};

use crate::util::read::read;

type XY = (usize, usize);

pub(crate) fn a(input: &str) -> usize {
    let (lines, start, end) = parse(input);
    bfs(
        lines,
        start,
        |node| node == &end,
        |node, next| next > node + 1,
    )
}

pub(crate) fn b(input: &str) -> usize {
    let (lines, _start, end) = parse(input);
    bfs(
        lines.clone(),
        end,
        |node| lines[node] == b'a',
        |node, next| next < node - 1,
    )
}

fn parse(input: &str) -> (HashMap<XY, u8>, XY, XY) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let out = read(input, |l| l.unwrap())
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i, j), c))
                .collect::<Vec<_>>()
        })
        .map(|(pos, c)| {
            let c = match c {
                'S' => {
                    start = pos;
                    b'a'
                }
                'E' => {
                    end = pos;
                    b'z'
                }
                x => x as u8,
            };
            (pos, c)
        })
        .collect();
    (out, start, end)
}

fn bfs<F, G>(map: HashMap<XY, u8>, from: XY, to: F, can_move: G) -> usize
where
    F: Fn(&XY) -> bool,
    G: Fn(u8, u8) -> bool,
{
    let mut q: VecDeque<(XY, usize)> = VecDeque::from([(from, 0)]);
    let mut visited = HashSet::from([(from)]);
    while let Some((node, cost)) = q.pop_front() {
        if to(&node) {
            return cost;
        }
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = (
                (node.0 as isize + di) as usize,
                (node.1 as isize + dj) as usize,
            );

            if !map.contains_key(&next) || visited.contains(&next) {
                continue;
            }

            if can_move(map[&node], map[&next]) {
                continue;
            }

            q.push_back((next, cost + 1));
            visited.insert(next);
        }
    }

    0
}
