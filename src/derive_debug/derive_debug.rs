#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p = Point { x: 2, y: 3 };
    println!(
        "Created a Point instance with coordinates: ({}, {})",
        p.x, p.y
    );
    println!("Debug representation of Point instance: {:?}", p);
}
