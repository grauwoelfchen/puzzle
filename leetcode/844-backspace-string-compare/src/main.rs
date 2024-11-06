use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let start = Instant::now();

        let (sr, _) = s.chars().rev().fold(("".to_string(), 0), |(r, p), c| {
            if c == '#' {
                (r, p + 1)
            } else if p > 0 {
                (r, p - 1)
            } else {
                (format!("{}{}", r, c), p)
            }
        });

        let (tr, _) = t.chars().rev().fold(("".to_string(), 0), |(r, p), c| {
            if c == '#' {
                (r, p + 1)
            } else if p > 0 {
                (r, p - 1)
            } else {
                (format!("{}{}", r, c), p)
            }
        });

        /*
        let mut sr = "".to_string();
        let mut sp = 0;
        for c in s.chars().rev() {
            if c == '#' {
                sp += 1;
            } else if sp > 0 {
                sp -= 1;
            } else {
                sr.push(c);
            }
        }

        let mut tr = "".to_string();
        let mut tp = 0;
        for c in t.chars().rev() {
            if c == '#' {
                tp += 1;
            } else if tp > 0 {
                tp -= 1;
            } else {
                tr.push(c);
            }
        }
        */

        let result = sr == tr;

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
        let s = "ab#c".to_string();
        let t = "ad#c".to_string();
        let result = Solution::backspace_compare(s, t);
        assert!(result);
    }

    #[test]
    fn test_example_2() {
        let s = "ab##".to_string();
        let t = "c#d#".to_string();
        let result = Solution::backspace_compare(s, t);
        assert!(result);
    }

    #[test]
    fn test_example_3() {
        let s = "a#c".to_string();
        let t = "b".to_string();
        let result = Solution::backspace_compare(s, t);
        assert!(!result);
    }

    #[test]
    fn test_example_4() {
        let s = "xywrrmp".to_string();
        let t = "xywrrmu#p".to_string();
        let result = Solution::backspace_compare(s, t);
        assert!(result);
    }

    #[test]
    fn test_example_113() {
        let s = "abc#".to_string();
        let t = "bac#".to_string();
        let result = Solution::backspace_compare(s, t);
        assert!(!result);
    }
}
