use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    iter::Map,
};

pub fn read(
    file: &str,
) -> Map<Lines<BufReader<File>>, fn(Result<String, std::io::Error>) -> String> {
    let file = File::open(file).expect("to open the file");
    io::BufReader::new(file).lines().map(|l| l.unwrap())
}
