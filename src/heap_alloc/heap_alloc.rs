fn main() {
    let a = Box::new(5);
    let b = 6;

    println!("a = {} is allocated in heap memory", a);
    println!(
        "b = {} is at location {}, in stack",
        b, &b as *const i32 as usize
    );
}
