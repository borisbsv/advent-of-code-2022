use std::collections::HashSet;

use crate::util::read::read;

pub(crate) fn a(input: &str) -> usize {
    move_snake(parse(input), 2).len()
}

pub(crate) fn b(input: &str) -> usize {
    move_snake(parse(input), 10).len()
}

fn parse(input: &str) -> impl Iterator<Item = ((i32, i32), i32)> {
    read(input, |l| {
        let tmp = l.unwrap();
        let (l, r) = tmp.split_once(' ').unwrap();
        let dir = match l {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            x => {
                println!("unexpected dir {}", x);
                (0, 0)
            }
        };
        (dir, r.parse::<i32>().unwrap())
    })
}

fn move_snake(actions: impl Iterator<Item = ((i32, i32), i32)>, len: usize) -> HashSet<(i32, i32)> {
    let mut snek = vec![(0, 0); len];

    let mut visited = HashSet::from([(0, 0)]);

    for (dir, times) in actions {
        for _ in 0..times {
            snek[0].0 += dir.0;
            snek[0].1 += dir.1;

            for i in 1..len {
                let delta_x = snek[i - 1].0 - snek[i].0;
                let delta_y = snek[i - 1].1 - snek[i].1;
                if delta_x.abs() < 2 && delta_y.abs() < 2 {
                    continue;
                }

                if delta_x != 0 {
                    snek[i].0 += delta_x / delta_x.abs();
                }

                if delta_y != 0 {
                    snek[i].1 += delta_y / delta_y.abs();
                }

                visited.insert(snek[len - 1]);
            }
        }
    }

    visited
}
