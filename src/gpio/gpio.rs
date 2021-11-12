pub struct GPIO {
    MODER: u32,
    BSRR: u32,
}

impl GPIO {
    pub fn new() -> GPIO {
        GPIO {MODER: 0, BSRR: 0}
    }

    pub fn get_mode(&self) -> u32 {
        self.MODER
    }
}

fn main() {
    let gpioa = GPIO::new();
    println!("gpioa = GPIO ({}, {})", GPIO::get_mode(&gpioa), gpioa.BSRR);
}