use itertools::Itertools;
use std::collections::HashSet;

use crate::util::read::read;

pub(crate) fn a(input: &str) -> usize {
    let lines = read(input, |l| {
        l.unwrap()
            .split(" -> ")
            .map(|c| {
                let coords = c.split_once(',').unwrap();
                Coord {
                    x: coords.0.parse().unwrap(),
                    y: coords.1.parse().unwrap(),
                }
            })
            .collect::<Vec<Coord>>()
    });
    simulate(to_cave(lines), None)
}

pub(crate) fn b(input: &str) -> usize {
    let lines = read(input, |l| {
        l.unwrap()
            .split(" -> ")
            .map(|c| {
                let coords = c.split_once(',').unwrap();
                Coord {
                    x: coords.0.parse().unwrap(),
                    y: coords.1.parse().unwrap(),
                }
            })
            .collect::<Vec<Coord>>()
    });
    let rocks = to_cave(lines);
    let ceiling = rocks.iter().map(|p| p.y).max().unwrap();
    simulate(rocks, Some(ceiling as usize + 2))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Default, PartialEq, Clone)]
enum Tile {
    Rock,
    Sand,
    #[default]
    Air,
}

impl Coord {
    fn neighbours(&self) -> [Coord; 3] {
        [
            Coord {
                x: self.x,
                y: self.y + 1,
            },
            Coord {
                x: self.x - 1,
                y: self.y + 1,
            },
            Coord {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    fn next(&self, cave: &[Vec<Tile>], floor: Option<usize>) -> Option<Coord> {
        if let Some(y) = floor {
            if (self.y + 1) == y.try_into().unwrap() {
                return None;
            }
        }
        self.neighbours()
            .into_iter()
            .find(|p| cave[p.y as usize][p.x as usize] == Tile::Air)
    }
}

fn simulate(rocks: HashSet<Coord>, floor: Option<usize>) -> usize {
    let sand_source = Coord { x: 500, y: 0 };
    let ceiling = rocks.iter().map(|p| p.y).max().unwrap();
    let width = ceiling * 4;
    let mut cave: Vec<Vec<Tile>> = vec![vec![Tile::Air; width as usize]; (ceiling + 2) as usize];

    for pos in rocks {
        cave[pos.y as usize][pos.x as usize] = Tile::Rock;
    }

    let mut memo = vec![sand_source];

    for i in 0.. {
        let mut sand = sand_source;
        while let Some(pos) = memo.pop() {
            if cave[pos.y as usize][pos.x as usize] == Tile::Air {
                sand = pos;
                break;
            }
        }

        memo.push(sand);

        while let Some(next) = sand.next(&cave, floor) {
            sand = next;
            memo.push(sand);
            if floor.is_none() && sand.y > ceiling {
                return i;
            }
        }
        cave[sand.y as usize][sand.x as usize] = Tile::Sand;
        if floor.is_some() && sand == sand_source {
            return i + 1;
        }
    }

    unreachable!()
}

fn to_cave(lines: impl Iterator<Item = Vec<Coord>>) -> HashSet<Coord> {
    lines
        .flat_map(|path| {
            path.iter()
                .tuple_windows()
                .flat_map(|(lhs, rhs)| {
                    let dx = rhs.x as i32 - lhs.x as i32;
                    let dy = rhs.y as i32 - lhs.y as i32;
                    let dir = Coord {
                        x: dx.signum(),
                        y: dy.signum(),
                    };

                    let steps = dx.abs().max(dy.abs());

                    (0..=steps).map(move |i| Coord {
                        x: lhs.x + dir.x * i,
                        y: lhs.y + dir.y * i,
                    })
                })
                .collect::<Vec<Coord>>()
        })
        .collect()
}
