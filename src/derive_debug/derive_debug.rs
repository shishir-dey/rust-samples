#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p = Point { x: 2, y: 3 };
    println!("p = {:?}", p);
}
