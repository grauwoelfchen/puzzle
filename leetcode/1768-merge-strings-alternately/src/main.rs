use std::str;

#[allow(dead_code)]
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        if word1.is_empty() {
            return word2;
        }
        if word2.is_empty() {
            return word1;
        }

        // let word1_bytes = word1.as_bytes();
        // let word2_bytes = word2.as_bytes();

        /*
        let mut result = "".to_string();
        let len = word1.len().max(word2.len());
        let mut i = 0;
        while i < len {
            if let Some(b) = word1.as_bytes().get(i) {
                result.push(*b as char);
            }
            if let Some(b) = word2.as_bytes().get(i) {
                result.push(*b as char);
            }
            i += 1;
        }
        result
        */

        let min = word1.len().min(word2.len());
        let mut res = String::with_capacity(word1.len() + word2.len());

        let mut i = 0;
        while i < min {
            if let Some(b) = word1.as_bytes().get(i) {
                res.push(*b as char);
            }
            if let Some(b) = word2.as_bytes().get(i) {
                res.push(*b as char);
            }
            i += 1;
        }

        res.extend(str::from_utf8(&word1.as_bytes()[i..]));
        res.extend(str::from_utf8(&word2.as_bytes()[i..]));
        res
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
        let word1 = "abc".to_string();
        let word2 = "pqr".to_string();
        let result: String = Solution::merge_alternately(word1, word2);
        assert_eq!(&result, "apbqcr");
    }

    #[test]
    fn test_example_2() {
        let word1 = "ab".to_string();
        let word2 = "pqrs".to_string();
        let result: String = Solution::merge_alternately(word1, word2);
        assert_eq!(&result, "apbqrs");
    }

    #[test]
    fn test_example_3() {
        let word1 = "abcd".to_string();
        let word2 = "pq".to_string();
        let result: String = Solution::merge_alternately(word1, word2);
        assert_eq!(&result, "apbqcd");
    }
}
