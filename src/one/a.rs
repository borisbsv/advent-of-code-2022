use std::fs::File;
use std::io::{self, BufRead};

pub fn a(input: &str) -> String {
    let file = File::open(input).expect("to open the file");
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let mut elf: i32 = 0;
    let mut max_elf: i32 = 0;

    for l in lines {
        match l.as_str() {
            "" => {
                if max_elf < elf {
                    max_elf = elf;
                }
                elf = 0;
            }
            n => elf += n.parse::<i32>().unwrap_or_default(),
        }
    }
    max_elf.to_string()
}
