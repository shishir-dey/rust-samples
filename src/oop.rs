#[derive(Debug)]
struct Human {
    name: u8,
    age: u8,
}

impl Human {
    fn learn(&self) {
        println!("[{}] Human::learn()", self as *const Human as usize);
    }
}

trait Living {
    fn breathe(&self);
    fn walk(&self);
    fn eat(&self);
}

impl Living for Human {
    fn breathe(&self) {
        println!("[{}] (Living)Human::breathe()", self as *const Human as usize);
    }
    fn walk(&self) {
        println!("[{}] (Living)Human::walk()", self as *const Human as usize);
    }
    fn eat(&self) {
        println!("[{}] (Living)Human::eat()", self as *const Human as usize);
    }
}

fn main() {
    let h1 = Human {name: 1, age: 2};
    h1.learn();
    h1.breathe();
    h1.walk();
    h1.eat();
}