pub mod geo {
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }

    pub fn origin() -> Point {
        Point {x: 0.0, y: 0.0}
    }
}

fn main() {
    use geo::*;
    let p = Box::new(Point {x: 0.0, y: 0.0});
    let p2 = origin();
    println!("p1 is at {} in heap", &p as *const Box<Point> as usize);
    println!("p2 is at {} in stack", &p2 as *const Point as usize);
}