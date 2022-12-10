use anyhow::Error;
use std::str::FromStr;

use crate::util::read::read;

enum Cmd {
    Noop,
    Addx(i32),
}
impl FromStr for Cmd {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Self::Noop);
        }
        Ok(Self::Addx(
            s.strip_prefix("addx ").unwrap().parse::<i32>().unwrap(),
        ))
    }
}

pub(crate) fn a(input: &str) -> i32 {
    let lines = read(input, |l| l.unwrap().parse::<Cmd>().unwrap());

    let mut counter: i32 = 1;
    let mut result: i32 = 0;
    let mut cycle: i32 = 0;
    let mut display: Vec<char> = Vec::with_capacity(280);

    let mut tick = |counter: &i32| {
        cycle += 1;
        let pos = cycle % 40;

        if cycle % 40 == 20 {
            result += counter * cycle as i32;
        }

        if *counter <= pos && pos <= counter + 2 {
            display.push('#');
        } else {
            display.push('.');
        }

        if cycle % 40 == 0 {
            display.push('\n');
        }
    };

    for cmd in lines {
        match cmd {
            Cmd::Noop => tick(&counter),
            Cmd::Addx(x) => {
                tick(&counter);
                tick(&counter);
                counter += x;
            }
        }
    }

    // print!("{}", display.into_iter().collect::<String>());
    result
}
