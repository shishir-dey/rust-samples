use std::thread;
use std::time;

fn main() {
    let t1 = thread::spawn(move || {
        let mut i = 1;
        while i <= 10 {
            println!("Thread 1 running!");
            thread::sleep(time::Duration::from_secs(1));
            i = i + 1;
        }
    });
    let t2 = thread::spawn(move || {
        let mut i = 1;
        while i <= 10 {
            println!("Thread 2 running!");
            thread::sleep(time::Duration::from_secs(1));
            i = i + 1;
        }
    });
    println!("{:?}", t1.join());
    println!("{:?}", t2.join());
}
