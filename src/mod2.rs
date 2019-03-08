#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn get_point(self) -> Point<T> {
        self
    }
}

pub mod arm {
    fn pid() {
        println!("ARMv7EM");
    }
    pub mod stm32 {
        pub fn ppid() {
            super::pid();
        }
    }
}

fn main() {
    let p = Point::<u8> {x: 2, y: 3};
    println!("{:?}", p.get_point());
    arm::stm32::ppid();
}