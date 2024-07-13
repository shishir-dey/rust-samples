fn main() {
    let x = 3;
    if x <= 0 {
        println!("{} is negative", x);
    } else if x == 0 {
        println!("x is equal to zero");
    } else {
        println!("{} is positive", x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_else() {
        let x = 3;
        if x <= 0 {
            assert_eq!(format!("{} is negative", x), "3 is negative");
        } else if x == 0 {
            assert_eq!(format!("x is equal to zero"), "x is equal to zero");
        } else {
            assert_eq!(format!("{} is positive", x), "3 is positive");
        }
    }
}
