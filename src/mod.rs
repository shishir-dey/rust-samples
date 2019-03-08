mod arm_v7em {
    pub mod cm4 {
        pub mod stm32f4 {
            pub mod stm32f407 {
                pub fn get_odr() -> u8 {
                    0
                }
            }
        }
    }
}

fn main() {
    if arm_v7em::cm4::stm32f4::stm32f407::get_odr() == 0 {
        println!("407VG");
    }
}