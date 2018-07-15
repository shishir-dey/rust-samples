fn sum(a: u8, b: u8) -> u8 {
    let c = a + b;
    c
}

#[test]
fn sum_testbench() {
    assert_eq!(sum(5, 5), 10);
}

fn main() {
    println!("Sum of 3 and 4 is {}", sum(3, 4));
}
