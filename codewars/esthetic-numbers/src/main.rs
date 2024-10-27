#[allow(dead_code)]
fn esthetic(num: u32) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    for r in 2..=10 {
        let mut s = String::new();
        let mut n = num;
        loop {
            if let Some(c) = char::from_digit(n % r, r) {
                s.insert(0, c);
            }
            n /= r;
            if n == 0 {
                break
            }
        }
        let chars: Vec<_> = s.chars().collect();
        if s != "0" && chars.windows(2).all(|v| {
            (v[0] as i32 - v[1] as i32).abs() == 1
        }) {
            result.push(r as u8);
        }
    }
    result
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
        let num = 10;
        assert_eq!(vec![2, 3, 8, 10], esthetic(num));

        let num = 23;
        assert_eq!(vec![3, 5, 7, 10], esthetic(num));

        let num = 666;
        assert_eq!(vec![8], esthetic(num));

        let num = 13;
        assert_eq!(vec![5, 6], esthetic(num));

        let num = 1;
        assert_eq!(vec![2, 3, 4, 5, 6, 7, 8, 9, 10], esthetic(num));

        let num = 9;
        assert_eq!(vec![4, 7, 9, 10], esthetic(num));
    }
}
