fn main() {
    let v = vec!["Sam", "Rick", "Joe"];

    for i in v.iter() {
        match i {
            &"Sam" => {
                println!("Sam is here");
            }
            _ => {
                println!("Somebody else");
            }
        }
    }
}
