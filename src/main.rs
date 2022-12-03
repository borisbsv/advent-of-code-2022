mod one;
mod three;
mod two;
mod util;

fn main() {
    println!("{}", one::solve::a("src/one/input"));
    println!("{}", one::solve::b("src/one/input"));
    println!("{}", two::solve::a("src/two/input"));
    println!("{}", two::solve::b("src/two/input"));
    println!("{}", three::solve::a("src/three/input"));
    println!("{}", three::solve::b("src/three/input"));
}
