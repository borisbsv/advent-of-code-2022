use std::{collections::HashSet, iter::Enumerate};

use crate::util::read::read;

pub(crate) fn a(input: &str) -> String {
    let lines = read(input, |l| {
        l.unwrap()
            .chars()
            .map(|c| -> i32 { c.to_digit(10).unwrap().try_into().unwrap() })
            .collect::<Vec<i32>>()
    })
    .collect::<Vec<Vec<i32>>>();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    let mut count: i32 = 0;
    lines.iter().enumerate().for_each(|(i, l)| {
        let mut max: i32 = -1;
        l.iter().enumerate().for_each(|(j, c)| {
            if c > &max {
                if visible.insert((i, j)) {
                    count += 1;
                };
                max = *c;
            }
        });
        max = -1;
        l.iter().rev().enumerate().for_each(|(j, c)| {
            if c > &max {
                if visible.insert((i, l.len() - j - 1)) {
                    count += 1;
                };
                max = *c;
            }
        })
    });

    let rotated_lines = matrix_transpose(&lines);
    rotated_lines.iter().enumerate().for_each(|(i, l)| {
        let mut max: i32 = -1;
        l.iter().enumerate().for_each(|(j, c)| {
            if c > &max {
                if visible.insert((j, i)) {
                    count += 1;
                };
                max = *c;
            }
        });
        max = -1;
        l.iter().rev().enumerate().for_each(|(j, c)| {
            if c > &max {
                if visible.insert((l.len() - j - 1, i)) {
                    count += 1;
                };
                max = *c;
            }
        })
    });

    count.to_string()
}

pub(crate) fn b(input: &str) -> String {
    let lines = read(input, |l| {
        l.unwrap()
            .chars()
            .map(|c| -> i32 { c.to_digit(10).unwrap().try_into().unwrap() })
            .collect::<Vec<i32>>()
    })
    .collect::<Vec<Vec<i32>>>();

    let rotated_lines = matrix_transpose(&lines);

    let mut max = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            let ss = scenic_score(&lines, &rotated_lines, (i, j));
            if max < ss {
                max = ss;
            }
        }
    }

    max.to_string()
}

fn matrix_transpose(m: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

fn scenic_score(
    matrix: &Vec<Vec<i32>>,
    rotated_matrix: &Vec<Vec<i32>>,
    coord: (usize, usize),
) -> u32 {
    let tree = matrix[coord.0][coord.1];
    // left
    let mut left: u32 = 0;
    let mut right: u32 = 0;
    let row = &matrix[coord.0];
    for i in (0..coord.1).rev() {
        left += 1;
        if row[i] >= tree {
            break;
        }
    }
    for i in coord.1 + 1..matrix[0].len() {
        right += 1;
        if row[i] >= tree {
            break;
        }
    }
    let mut top: u32 = 0;
    let mut bottom: u32 = 0;
    let col = &rotated_matrix[coord.1];
    for i in (0..coord.0).rev() {
        top += 1;
        if col[i] >= tree {
            break;
        }
    }
    for i in coord.0 + 1..rotated_matrix[0].len() {
        bottom += 1;
        if col[i] >= tree {
            break;
        }
    }
    left * right * top * bottom
}
