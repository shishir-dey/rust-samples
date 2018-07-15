fn get_array() -> [u8; 5] {
    [6, 7, 8, 9, 10]
}

fn main() {
    let ar: [u8; 5] = [1, 2, 3, 4, 5];
    println!("Array = {:?}", ar);

    println!("Array2 = {:?}", get_array());
}
