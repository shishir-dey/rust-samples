fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> i32 {
    if b != 0 {
        a / b
    } else {
        panic!("Cannot divide by zero")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-2, 3), 1);
        assert_eq!(add(2, -3), -1);
        assert_eq!(add(-2, -3), -5);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(5, 3), 2);
        assert_eq!(sub(-5, 3), -8);
        assert_eq!(sub(5, -3), 8);
        assert_eq!(sub(-5, -3), -2);
        assert_eq!(sub(0, 0), 0);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(2, 3), 6);
        assert_eq!(mul(-2, 3), -6);
        assert_eq!(mul(2, -3), -6);
        assert_eq!(mul(-2, -3), 6);
        assert_eq!(mul(0, 3), 0);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(6, 3), 2);
        assert_eq!(div(-6, 3), -2);
        assert_eq!(div(6, -3), -2);
        assert_eq!(div(-6, -3), 2);
        assert_eq!(div(0, 3), 0);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_div_by_zero() {
        div(1, 0);
    }
}

fn main() {
    let a = 10;
    let b = 5;

    println!("{} + {} = {}", a, b, add(a, b));
    println!("{} - {} = {}", a, b, sub(a, b));
    println!("{} * {} = {}", a, b, mul(a, b));
    println!("{} / {} = {}", a, b, div(a, b));
}
