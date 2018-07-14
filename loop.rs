fn main() {
    let mut count = 5;
    loop {
        println!("count = {}", count);
        count -= 1;

        if count == 0 {
            break;
        }
    }
}
