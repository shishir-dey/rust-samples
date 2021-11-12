use std::env;

fn main() {
    for i in env::args() {
        println!("{}", i);
    }
    let l = [1, 2, 3];
    println!("l[0] = {}", l[0]);
}