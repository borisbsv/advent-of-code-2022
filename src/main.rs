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
}
struct Result(String, std::time::Duration);
impl fmt::Display for Result {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "result: {:20}| took: {:?}", self.0, self.1)
    }
}

fn bench(f: fn(&str) -> String, input: &str) -> Result {
    use std::time::Instant;
    let now = Instant::now();
    let res = f(input);

    Result(res, now.elapsed())
}
