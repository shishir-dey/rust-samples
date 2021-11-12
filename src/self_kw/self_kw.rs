#[derive(Debug)]
struct Human {
    name: u8,
    age: u8,
}

impl Human {
    fn new() -> Self {
        Human {name: 1, age: 2}
    }
    fn ret(&self) {
        println!("self.name = {}, self.age = {}", &self.name as *const u8 as usize, &self.age as *const u8 as usize);
    }
}

fn main() {
    let h1 = Human::new();
    println!("h1 = {:?}", h1);
    Human::ret(&h1);
}