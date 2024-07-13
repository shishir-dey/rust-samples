#[allow(dead_code)]
#[allow(non_snake_case)]
struct GPIO {
    MODER: u32,
    BSRR: u32,
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
impl GPIO {
    fn SetPin(g: GPIO, pin: u8) -> bool {
        true
    }
}

fn main() {
    let gpioa = GPIO { MODER: 0, BSRR: 0 };
    GPIO::SetPin(gpioa, 1);
}
