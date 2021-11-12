fn get_tuple() -> (u8, u8, u8, u8, u8) {
    (6, 7, 8, 9, 10)
}

fn main() {
    let tup = (1, 2, 3, 4, 5);
    println!("tup = {:?}", tup);

    println!("tup2 = {:?}", get_tuple());
}
