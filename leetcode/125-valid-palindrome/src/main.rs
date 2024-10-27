use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let start = Instant::now();

        let filtered: String = s
            .chars()
            .filter_map(|c| {
                if !c.is_ascii_alphanumeric() {
                    None
                } else {
                    Some(c.to_lowercase().to_string())
                }
            })
            .collect();

        let n = filtered.len() / 2;
        let bytes = filtered.as_bytes();

        let result = bytes[..n]
            .iter()
            .zip(bytes[n..].iter().rev())
            .all(|(a, b)| a == b);

        println!("{:.2?}", start.elapsed());
        result
    }
}

pub struct Solution;

#[allow(dead_code)]
fn main() {
    unimplemented!();
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "A man, a plan, a canal: Panama".to_string();
        let result = Solution::is_palindrome(s);
        assert!(result);
    }

    #[test]
    fn test_example_2() {
        let s = "race a car".to_string();
        let result = Solution::is_palindrome(s);
        assert!(!result);
    }

    #[test]
    fn test_example_3() {
        let s = " ".to_string();
        let result = Solution::is_palindrome(s);
        assert!(result);
    }

    #[test]
    fn test_example_4() {
        let s = "0P".to_string();
        let result = Solution::is_palindrome(s);
        assert!(!result);
    }
}
