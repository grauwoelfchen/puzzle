// use std::str;

#[allow(dead_code)]
impl Solution {
    // => 1. iterate (0..(len - 1)).rev() and replace all
    //    2. abab -> abababab -> a bababa b ---> 1 2 1 = 4 (x * 2)
    fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        if n < 2 {
            return false;
        }

        for i in (1..n).rev() {
            if n % i == 0 && s.replace(&s[0..i as usize], "") == "" {
                return true;
            }
        }
        false
    }
}

struct Solution;

fn main() {
    unimplemented!();
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "abab".to_string();
        let result = Solution::repeated_substring_pattern(s);
        assert_eq!(true, result);
    }

    #[test]
    fn test_example_2() {
        let s = "aba".to_string();
        let result = Solution::repeated_substring_pattern(s);
        assert_eq!(false, result);
    }

    #[test]
    fn test_example_3() {
        let s = "abcabcabcabc".to_string();
        let result = Solution::repeated_substring_pattern(s);
        assert_eq!(true, result);
    }

    #[test]
    fn test_bug_1() {
        let s = "aaaa".to_string();
        let result = Solution::repeated_substring_pattern(s);
        assert_eq!(true, result);
    }

    #[test]
    fn test_bug_2() {
        let s = "abaababaab".to_string();
        let result = Solution::repeated_substring_pattern(s);
        assert_eq!(true, result);
    }

    #[test]
    fn test_bug_3() {
        let s = "bb".to_string();
        let result = Solution::repeated_substring_pattern(s);
        assert_eq!(true, result);
    }
}
