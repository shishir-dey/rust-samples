fn inc(n: &mut u8) {
    *n = *n + 1
}

mod gpio;

fn main() {
    let mut a: u8 = 4_u8;
    println!("a = {}", a);
    inc(&mut a);
    println!("a = {}", a);

    let gpioa = gpio::GPIO::new();
    println!("get_mode(gpioa) = {}", gpio::GPIO::get_mode(&gpioa));

    let s: u8 = 4;
    let ref ps = s;
    println!("{} is at {}", s, ps as *const u8 as usize);
}