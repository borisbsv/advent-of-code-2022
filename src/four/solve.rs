use crate::util::read::read;

pub(crate) fn a(input: &str) -> i32 {
    let assgmts = parse(input);
    assgmts
        .map(|(lhs, rhs)| {
            i32::from((lhs.0 <= rhs.0 && lhs.1 >= rhs.1) || (rhs.0 <= lhs.0 && rhs.1 >= lhs.1))
        })
        .sum()
}

pub(crate) fn b(input: &str) -> i32 {
    parse(input)
        .map(|(lhs, rhs)| i32::from(lhs.0 <= rhs.1 && rhs.0 <= lhs.1))
        .sum::<i32>()
}

fn parse(input: &str) -> impl Iterator<Item = ((i32, i32), (i32, i32))> {
    read(input, |l| {
        let each = l.as_ref().unwrap().split_once(',').unwrap();

        let mut lhs: Vec<i32> = each
            .0
            .split('-')
            .map(|d| d.parse::<i32>().unwrap())
            .collect();
        let mut rhs: Vec<i32> = each
            .1
            .split('-')
            .map(|d| d.parse::<i32>().unwrap())
            .collect();
        lhs.sort();
        rhs.sort();

        (
            (*lhs.first().unwrap(), *lhs.get(1).unwrap()),
            (*rhs.first().unwrap(), *rhs.get(1).unwrap()),
        )
    })
}
