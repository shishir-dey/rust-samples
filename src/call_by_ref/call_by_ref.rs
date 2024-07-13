fn inc(n: &mut u8) {
    let mut t = *n;
    t = t + 1;
    *n = t;
}

fn main() {
    let mut a: u8 = 5;
    println!("a = {}", a);

    inc(&mut a);
    println!("a is {}", a);
}
