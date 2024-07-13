use std::time::{SystemTime, UNIX_EPOCH};
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn get_rand_day() -> Day {
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let random_number = since_epoch.as_secs() % 7;

        match random_number {
            0 => Day::Monday,
            1 => Day::Tuesday,
            2 => Day::Wednesday,
            3 => Day::Thursday,
            4 => Day::Friday,
            5 => Day::Saturday,
            _ => Day::Sunday,
        }
    }
    fn match_day(self) {
        match self {
            Day::Monday => {
                println!("Today is Monday!");
            }
            Day::Tuesday => {
                println!("Today is Tuesday!");
            }
            Day::Wednesday => {
                println!("Today is Wednesday!");
            }
            Day::Thursday => {
                println!("Today is Thursday!");
            }
            Day::Friday => {
                println!("Today is Friday!");
            }
            Day::Saturday => {
                println!("Today is Saturday!");
            }
            Day::Sunday => {
                println!("Today is Sunday!");
            }
        }
    }
}

fn main() {
    let d = Day::get_rand_day();
    d.match_day(); // Call match_day on the instance `d`
}
