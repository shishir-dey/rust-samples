fn main() {

    #[allow(dead_code)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    let today = Day::Sunday;

    match today {
        Day::Monday => { println!("Today is Monday"); },
        Day::Tuesday => { println!("Today is Tuesday"); },
        Day::Wednesday => { println!("Today is Wednesday"); },
        Day::Thursday => { println!("Today is Thursday"); },
        Day::Friday => { println!("Today is Friday"); },
        Day::Saturday => { println!("Today is Saturday"); },
        Day::Sunday => { println!("Today is Sunday"); },
    }
}
