pub struct GPIO {
    moder: u32,
    bsrr: u32,
}

impl GPIO {
    pub fn new() -> GPIO {
        GPIO { moder: 0, bsrr: 0 }
    }

    pub fn get_mode(&self) -> u32 {
        self.moder
    }
}

fn main() {
    let gpioa = GPIO::new();
    println!("gpioa = GPIO ({}, {})", GPIO::get_mode(&gpioa), gpioa.bsrr);
}
