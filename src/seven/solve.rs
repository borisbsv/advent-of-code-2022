use anyhow::Error;
use std::{collections::HashMap, str::FromStr};

use crate::util::read::read;

#[derive(Debug, Default, Clone)]
struct File {
    size: u32,
}

#[derive(Debug, Clone)]
enum Cmd {
    LS,
    CD(String),
}

#[derive(Debug, Clone)]
enum Line {
    Cmd(Cmd),
    Dir(String),
    File(File),
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cmd) = s.strip_prefix("$ ") {
            if let Some(dir) = cmd.trim().strip_prefix("cd ") {
                Ok(Self::Cmd(Cmd::CD(dir.to_string())))
            } else {
                Ok(Self::Cmd(Cmd::LS))
            }
        } else if let Some(dir) = s.strip_prefix("dir ") {
            Ok(Line::Dir(dir.to_string()))
        } else {
            let (size, _) = s.split_once(' ').unwrap();
            Ok(Line::File(File {
                size: size.parse().unwrap(),
            }))
        }
    }
}

#[derive(Debug, Default)]
struct DirTree {
    children: HashMap<String, DirTree>,
    files: Vec<File>,
    size: Option<u32>,
}

impl DirTree {
    fn size(&mut self) {
        for d in self.children.values_mut() {
            d.size()
        }
        self.size = Some(
            self.files.iter().map(|f| f.size).sum::<u32>()
                + self.children.values().map(|d| d.size.unwrap()).sum::<u32>(),
        )
    }
}

fn move_tree<'a>(tree: &'a mut DirTree, path: &'_ [String]) -> &'a mut DirTree {
    let mut temp = tree;
    for dir in path.iter().skip(1) {
        temp = temp.children.get_mut(dir).unwrap();
    }
    temp
}

fn make_tree(lines: impl Iterator<Item = Line>) -> DirTree {
    let mut curr_path = Vec::new();
    let mut tree = DirTree {
        children: HashMap::new(),
        files: vec![],
        size: None,
    };

    let mut curr_tree = &mut tree;

    for line in lines {
        match line {
            Line::Cmd(Cmd::LS) => {}
            Line::Cmd(Cmd::CD(next_dir)) => {
                if &next_dir == ".." {
                    curr_path.pop().unwrap();
                } else {
                    curr_path.push(next_dir);
                }
                curr_tree = move_tree(&mut tree, &curr_path);
            }
            Line::Dir(dir) => {
                curr_tree.children.insert(dir, Default::default());
            }
            Line::File(file) => curr_tree.files.push(file),
        }
    }

    tree
}

fn recurse_value(root: &DirTree) -> u32 {
    let subdir_total: u32 = root.children.values().map(recurse_value).sum();
    subdir_total + root.size.filter(|s| s <= &100_000).unwrap_or_default()
}

pub(crate) fn a(input: &str) -> String {
    let mut tree = make_tree(read(input, |l| l.unwrap().parse::<Line>().unwrap()));
    tree.size();
    recurse_value(&tree).to_string()
}

pub(crate) fn b(input: &str) -> String {
    let mut tree = make_tree(read(input, |l| l.unwrap().parse::<Line>().unwrap()));
    tree.size();

    let mut min: u32 = 70_000_000;
    let needed: u32 = 30_000_000 - (min - tree.size.unwrap());

    let mut dirs = Vec::from_iter(tree.children.values());
    while let Some(d) = dirs.pop() {
        let size = d.size.unwrap();
        if size < min && size >= needed {
            min = size;
        }
        dirs.extend(d.children.values());
    }

    min.to_string()
}
