#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let p = Point {x:3.2, y: 4.5};
    println!("p= {:?}", p);

    let Point{x: i, y: j} = p;
    println!("i = {} j = {}", i, j);

}