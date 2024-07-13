use std::collections::HashMap;

fn main() {
    let mut db = HashMap::new();

    db.insert("A", "1");
    db.insert("B", "2");
    db.insert("C", "3");

    for (k, v) in &db {
        println!("Key: {} Value: {}", k, v);
    }
}
