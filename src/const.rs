static DEVICE: &str = "I2C OMAP Adapter";
const SLAVE_ADDR: u8 = 0x50;

fn main() {
    println!("Device = {}", DEVICE);
    println!("SLAVE_ADDR = {}", SLAVE_ADDR);
}