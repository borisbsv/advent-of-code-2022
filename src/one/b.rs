use std::fs::File;
use std::io::{self, BufRead};

pub fn b(input: &str) -> String {
    let file = File::open(input).expect("to open the file");
    let lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let mut elf: i32 = 0;
    let mut max_elfs: [i32; 3] = [0; 3];

    for l in lines {
        match l.as_str() {
            "" => {
                for (i, _) in max_elfs.iter().enumerate() {
                    if elf > max_elfs[i] {
                        max_elfs[i] = elf;
                        max_elfs.sort();
                        break;
                    }
                }
                elf = 0;
            }
            n => elf += n.parse::<i32>().unwrap_or_default(),
        }
    }
    max_elfs.iter().sum::<i32>().to_string()
}
