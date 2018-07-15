fn main() {
    struct Complex {
        re: u32,
        im: u32,
    }
    trait ComplexMethods {
        fn get_re(&self) -> u32;
        fn get_im(&self) -> u32;
    }
    impl ComplexMethods for Complex {
        fn get_re(&self) -> u32 {
            self.re
        }
        fn get_im(&self) -> u32 {
            self.im
        }
    }

    let cplx_1 = Complex { re: 1, im: 1, };
    println!("Complex Number re: {} im: {}", cplx_1.get_re(), cplx_1.get_im());
}
