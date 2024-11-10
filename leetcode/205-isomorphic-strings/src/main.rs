use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let start = Instant::now();

        let sm: HashMap<char, char> = s.chars().zip(t.chars()).collect();
        let sn: String = s.chars().map(|c| sm.get(&c).unwrap()).collect();

        let tm: HashMap<char, char> = t.chars().zip(s.chars()).collect();
        let tn: String = t.chars().map(|c| tm.get(&c).unwrap()).collect();

        let result = sn == t && tn == s;

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
        let s = "egg".to_string();
        let t = "add".to_string();
        let result = Solution::is_isomorphic(s, t);
        assert!(result);
    }

    #[test]
    fn test_example_2() {
        let s = "foo".to_string();
        let t = "bar".to_string();
        let result = Solution::is_isomorphic(s, t);
        assert!(!result);
    }

    #[test]
    fn test_example_3() {
        let s = "paper".to_string();
        let t = "title".to_string();
        let result = Solution::is_isomorphic(s, t);
        assert!(result);
    }

    #[test]
    fn test_example_39() {
        let s = "badc".to_string();
        let t = "baba".to_string();
        let result = Solution::is_isomorphic(s, t);
        assert!(!result);
    }
}
