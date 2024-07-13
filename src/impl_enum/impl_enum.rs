#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
}

impl Day {
    fn print_day(d: Day) {
        use Day::*;
        match d {
            Monday => {
                println!("Today is {:?}", d)
            }
            _ => {
                println!("Today is {:?}", d);
            }
        }
    }
}

fn main() {
    use Day::*;
    let d = Monday;
    Day::print_day(d);
}
