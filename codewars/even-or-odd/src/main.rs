#[allow(dead_code)]
fn even_or_odd(number: i32) -> &'static str {
    match number % 2 {
        0 => "Even",
        _ => "Odd",
    }
}

fn main() {
    unimplemented!();
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let n = 0;
        assert_eq!("Even", even_or_odd(n));

        let n = 2;
        assert_eq!("Even", even_or_odd(n));

        let n = 1;
        assert_eq!("Odd", even_or_odd(n));

        let n = 7;
        assert_eq!("Odd", even_or_odd(n));

        let n = -1;
        assert_eq!("Odd", even_or_odd(n));
    }
}
