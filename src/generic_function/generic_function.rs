fn get_data<T>(a: T) -> T {
    a
}

fn main() {
    let pi: f32 = get_data::<f32>(3.14);
    println!("pi = {}", pi);
}
