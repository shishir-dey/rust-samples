fn get_list() -> [u8; 4] {
    [1, 2, 3, 4]
}

fn main() {
    let l = get_list();
    println!("l = {:?}", l);
}