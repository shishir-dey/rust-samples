use std::mem;

#[derive(Debug)]
struct Dim<E, X> {
    x: E,
    y: X,
}

fn main() {
    let s = Dim::<f32, f64>{x: 3.14, y: 6.23};
    println!("s = {:?}", s);
    println!("size of Dim is {}", mem::size_of::<Dim<u8, u8>>());
}