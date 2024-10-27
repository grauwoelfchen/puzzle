#[allow(dead_code)]
impl Solution {
    fn is_palindrome(x: i32) -> bool {
        /*
        let s = x.to_string();
        let c = s.chars();
        c.clone().zip(c.rev()).all(|(a, b)| a == b)
        */

        let s = x.to_string();
        let n = s.len() / 2;
        let b = s.as_bytes();

        b[..n].iter().zip(b[n..].iter().rev()).all(|(a, b)| a == b)
    }
}

struct Solution;

fn main() {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let x = 121;
        let result = Solution::is_palindrome(x);
        assert!(result);
    }

    #[test]
    fn test_example_2() {
        let x = -121;
        let result = Solution::is_palindrome(x);
        assert!(!result);
    }

    #[test]
    fn test_example_3() {
        let x = 10;
        let result = Solution::is_palindrome(x);
        assert!(!result);
    }
}
