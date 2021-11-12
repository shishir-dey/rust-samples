#[derive(Debug, Clone)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new() -> Self {
        Point {x: 0, y: 0}
    }
    fn rand(mut self) -> Self {
        self.x = 3;
        self.y = 4;
        self
    }
}

fn main() {
    let p = Point::new().rand();
    println!("p = {:?}", p);
    let p2 = p.clone();
    drop(p);
    println!("p2 = {:?}", p2);
}