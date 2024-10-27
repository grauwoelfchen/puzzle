#[allow(dead_code)]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let result = multiply(3, 5);
        assert_eq!(15, result);
    }
}
