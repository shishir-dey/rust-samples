fn main() {
    let a: u32 = 5;
    let b: &u32 = &a;

    println!("a = {} is at {}", a, b as *const u32 as usize);
}
