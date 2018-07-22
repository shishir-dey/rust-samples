fn main() {
    let args = std::env::args();

    print!("Args = [");
    for arg in args {
        print!(" {} ", arg);
    }
    println!("]");
}
