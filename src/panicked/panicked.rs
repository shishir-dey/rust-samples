fn div(x: u8, y: u8) -> Result<u8, u8> {
    if y == 0 {
        Err(1)
    }
    else {
        Ok(x/y)
    }
}

fn div2(x: u8, y: u8) {
    if y == 0 {
        panic!("y == 0");
    }
    else {
        println!("{}", x / y);
    }
}

fn main() {
    println!("{:?}", div(2, 2));
    div2(2, 2);
}