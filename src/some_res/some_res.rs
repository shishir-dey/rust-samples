#[derive(Debug)]
enum Error {
    DivideByZero,
    MismatchTypes,
}

fn div(x: f32, y: f32) -> Result<f32, Error> {
    if y == 0.0 {
        Err(Error::DivideByZero)
    } else {
        Ok(x / y)
    }
}

fn gift(g: Option<u8>) {
    match g {
        Some(x) => match x {
            1 => {
                println!("{}", 1);
            }
            _ => {
                println!("{}", 0);
            }
        },
        None => {
            println!("No gift! Huh!");
        }
    }
}

fn main() {
    println!("{:?}", div(2.0, 2.0));
    println!("{:?}", div(2.0, 0.0));
    gift(None);
    gift(Some(1));
    gift(Some(3));
    let e: Error = Error::MismatchTypes;
    println!("{:?}", e);
}
