use crate::util::read::read;

pub(crate) fn a(input: &str) -> String {
    solve(input, 4)
}

pub(crate) fn b(input: &str) -> String {
    solve(input, 14)
}

fn solve(input: &str, count: usize) -> String {
    let line: Vec<_> = read(input, |l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .to_vec();
    let iter = line.iter();

    let mut binding = iter.clone().take(count).collect::<Vec<_>>();
    let last_n = binding.as_mut_slice();

    for (i, c) in iter.enumerate() {
        if !(1..last_n.len()).any(|i| last_n[i..].contains(&last_n[i - 1])) {
            return (i).to_string();
        }
        last_n[0] = c;
        last_n.rotate_left(1);
    }
    "".to_string()
}
