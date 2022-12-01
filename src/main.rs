mod one;

use one::a::a;
use one::b::b;

fn main() {
    println!("{}", a("src/one/input"));
    println!("{}", b("src/one/input"));
}
