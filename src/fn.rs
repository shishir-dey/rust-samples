fn f(x: u8) -> u8 {
    x + 1
}

fn g(x: u8) -> u8 {
    x * x
}
 enum Day {
     Monday,
     Tuesday,
 }

 impl Day {
     fn get_rand_day() -> Day {
         Day::Monday
     }
     fn match_day(self) {
         match self {
             Day::Monday => {
                 println!("Today is Monday!");
             }
             _ => {
                 println!("Some other day.");
             }
         }
     }
 }

fn main() {    
    println!("gof(2) = {}", g(f(2)));
    let d = Day::get_rand_day();
    Day::match_day(d);
}