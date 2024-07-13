fn main() {
    let mut count = 0;
    'outer: loop {
        loop {
            count = count + 1;
            println!("count = {}", count);
            if count == 10 {
                break 'outer;
            }
        }
    }
}
