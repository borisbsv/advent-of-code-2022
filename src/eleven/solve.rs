use crate::util::read::read;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: Test,
}

#[derive(Clone, Debug)]
struct Test {
    by: u64,
    yes: u64,
    no: u64,
}

#[derive(Default, Clone, Debug)]
struct Operation {
    rhs: u64,
    op: Op,
}

impl Operation {
    fn r#do(&self, to: u64) -> u64 {
        match self.op {
            Op::Sum => self.rhs + to,
            Op::Multiply => self.rhs * to,
            Op::Square => to * to,
            Op::Noop => to,
        }
    }
}

#[derive(Default, Clone, Debug)]
enum Op {
    Sum,
    Multiply,
    Square,
    #[default]
    Noop,
}

pub(crate) fn a(input: &str) -> u64 {
    let monkeys = parse(input);

    let mut inspections: Vec<u64> = vec![0; monkeys.len()];

    let mut items: Vec<Vec<u64>> = monkeys.iter().map(|monkey| monkey.items.clone()).collect();
    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let these_items: Vec<u64> = items[i].drain(..).collect();

            for item in these_items {
                let new = monkey.op.r#do(item) / 3;

                let next_monkey = if new % monkey.test.by == 0 {
                    monkey.test.yes
                } else {
                    monkey.test.no
                };

                items[next_monkey as usize].push(new);
                inspections[i] += 1;
            }
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

pub(crate) fn b(input: &str) -> u64 {
    let monkeys = parse(input);
    let new_relief = monkeys.iter().map(|m| m.test.by as u64).product::<u64>() as u64;
    let mut inspections: Vec<u64> = vec![0; monkeys.len()];

    let mut items: Vec<Vec<u64>> = monkeys.iter().map(|monkey| monkey.items.to_vec()).collect();
    for _ in 0..10_000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let these_items: Vec<u64> = items[i].drain(..).collect();

            for item in these_items {
                let new = monkey.op.r#do(item) % new_relief;

                let next_monkey = if new % monkey.test.by == 0 {
                    monkey.test.yes
                } else {
                    monkey.test.no
                };

                items[next_monkey as usize].push(new);
                inspections[i] += 1;
            }
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

fn parse(input: &str) -> Vec<Monkey> {
    let lines = read(input, |l| -> String { l.unwrap() })
        .collect::<Vec<_>>()
        .split(|l| l.is_empty())
        .map(|arr| -> Monkey {
            let items: Vec<u64> = arr[1]
                .chars()
                .skip(18)
                .collect::<String>()
                .split(", ")
                .map(|each| each.parse::<u64>().unwrap())
                .collect();

            let mut op_s = arr[2][23..].chars();
            let op = match op_s.next().unwrap() {
                '+' => Operation {
                    rhs: op_s.skip(1).collect::<String>().parse().unwrap(),
                    op: Op::Sum,
                },
                '*' => match op_s.skip(1).collect::<String>().parse::<u64>() {
                    Ok(multiplier) => Operation {
                        rhs: multiplier,
                        op: Op::Multiply,
                    },
                    _ => Operation {
                        rhs: 0,
                        op: Op::Square,
                    },
                },
                _ => Default::default(),
            };

            let test: Test = Test {
                by: arr[3].split(' ').last().unwrap().parse::<u64>().unwrap(),
                yes: arr[4].split(' ').last().unwrap().parse::<u64>().unwrap(),
                no: arr[5].split(' ').last().unwrap().parse::<u64>().unwrap(),
            };

            Monkey { items, op, test }
        })
        .collect();
    lines
}
