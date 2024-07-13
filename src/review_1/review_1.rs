use std::mem;

#[derive(Debug)]
struct Dim<E, X> {
    x: E,
    y: X,
}

fn main() {
    let s = Dim::<f32, f64> { x: 3.14, y: 6.23 };
    println!("The Dim struct instance: {:?}", s);
    println!("The value of x: {}, The value of y: {}", s.x, s.y);
    println!(
        "The size of the Dim struct in memory: {} bytes",
        mem::size_of::<Dim<f32, f64>>()
    );
}
