fn main() {
    let x = 3;
    if x <= 0 {
        println!("{} is negative", x);
    } else if x == 0 {
        println!("x is equal to zero");
    } else {
        println!("{} is positive", x);
    }
}

fn test_if_else() {
    assert_eq!(1, 1);
}
