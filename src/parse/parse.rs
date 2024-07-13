fn main() {
    let s = "5";
    let s2 = s.parse::<u8>().unwrap();
    let s3 = s2.to_string();
    println!("s = {}", s3);
}
