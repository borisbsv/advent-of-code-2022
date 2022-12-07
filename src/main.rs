use std::fmt;

mod five;
mod four;
mod one;
mod seven;
mod six;
mod three;
mod two;

mod util;

fn main() {
    println!("┌{}┬{}┬{}┐", "─".repeat(14), "─".repeat(29), "─".repeat(17));
    println!("{}", bench(one::solve::a, "src/one/input"));
    println!("{}", bench(one::solve::b, "src/one/input"));
    println!("{}", bench(two::solve::a, "src/two/input"));
    println!("{}", bench(two::solve::b, "src/two/input"));
    println!("{}", bench(three::solve::a, "src/three/input"));
    println!("{}", bench(three::solve::b, "src/three/input"));
    println!("{}", bench(four::solve::a, "src/four/input"));
    println!("{}", bench(four::solve::b, "src/four/input"));
    println!("{}", bench(five::solve::a, "src/five/input"));
    println!("{}", bench(five::solve::b, "src/five/input"));
    println!("{}", bench(six::solve::a, "src/six/input"));
    println!("{}", bench(six::solve::b, "src/six/input"));
    println!("{}", bench(seven::solve::a, "src/seven/input"));
    println!("{}", bench(seven::solve::b, "src/seven/input"));
    println!("└{}┴{}┴{}┘", "─".repeat(14), "─".repeat(29), "─".repeat(17));
}
struct Result(String, String, std::time::Duration);
impl fmt::Display for Result {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "│day {:10}│ result: {:20}│ took: {:10?}│",
            self.0, self.1, self.2
        )
    }
}

fn bench(f: fn(&str) -> String, input: &str) -> Result {
    use std::time::Instant;
    let now = Instant::now();
    let res = f(input);
    let elapsed = now.elapsed();

    Result(input.split('/').nth(1).unwrap().to_string(), res, elapsed)
}
