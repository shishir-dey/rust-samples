fn p(s: &[u8]) {
    println!("{:?}", s);
}

fn main() {
    p("Hello".as_bytes());
    let p2: u8 = "2".parse::<u8>().unwrap();
    println!("p2 = {}", p2);
}