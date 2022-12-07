use crate::util::read::read;

pub(crate) fn a(input: &str) -> String {
    let lines = read(input, |l| l.unwrap());
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
            n => elf += n.parse::<i32>().unwrap(),
        }
    }
    max_elf.to_string()
}

pub(crate) fn b(input: &str) -> String {
    let lines = read(input, |l| l.unwrap());
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
            n => elf += n.parse::<i32>().unwrap(),
        }
    }
    max_elfs.iter().sum::<i32>().to_string()
}
