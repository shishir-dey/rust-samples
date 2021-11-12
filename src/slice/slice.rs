fn print_slice(s: &[u8]) {
    let mut i = 0;
    while i < s.len() {
        print!("{} ", s[i]);
        i = i + 1;
    }
    println!("");
}

fn main() {
    let ar: [u8; 5] = [1, 2, 3, 4, 5];

    print_slice(&ar[0 .. 2]);
}