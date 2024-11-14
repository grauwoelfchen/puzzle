use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    // 1 <= s.length, t.length <= 5 * 10^4
    // lowercase
    pub fn is_anagram(s: String, t: String) -> bool {
        let start = Instant::now();

        if s.len() != t.len() {
            return false;
        }

        // eg. 11.90ms, 5.53ms
        let mut sc = s.chars().collect::<Vec<char>>();
        sc.sort();
        let ss = sc.iter().collect::<String>();

        let mut mc = t.chars().collect::<Vec<char>>();
        mc.sort();
        let st = mc.iter().collect::<String>();

        let result = ss == st;

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
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let result = Solution::is_anagram(s, t);
        assert!(result);
    }

    #[test]
    fn test_example_2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        let result = Solution::is_anagram(s, t);
        assert!(!result);
    }

    #[test]
    fn test_example_32() {
        let s = "aa".to_string();
        let t = "a".to_string();
        let result = Solution::is_anagram(s, t);
        assert!(!result);
    }

    #[test]
    fn test_example_36() {
        let s = "fe".to_string();
        let t = "ja".to_string();
        let result = Solution::is_anagram(s, t);
        assert!(!result);
    }

    #[test]
    fn test_example_38() {
        let s = "ac".to_string();
        let t = "bb".to_string();
        let result = Solution::is_anagram(s, t);
        assert!(!result);
    }
}
