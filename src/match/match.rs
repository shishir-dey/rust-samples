fn do_match(n: u8) -> bool {
    match n {
        k @ 90..=99 => {
            println!("Percent = {} Grade : {grade}", k, grade = 'A');
            true
        }
        _ => false,
    }
}
fn main() {
    do_match(95);
}
