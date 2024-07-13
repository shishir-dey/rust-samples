fn main() {
    let mut s = String::from("This is a string");
    let mut s2 = "This is another string".to_string();

    println!("{}\n{}", s, s2);

    s.remove(0);
    s2.push('.');

    println!("{}\n{}", s, s2);
}

/* Refer to https://doc.rust-lang.org/std/string/struct.String.html */
