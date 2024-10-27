use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let start = Instant::now();

        /*
        if let Some(i) = haystack.find(&needle) {
            return i as i32;
        }
        -1
        */

        let nlen = needle.len();
        let sset: Vec<(i32, String)> = haystack
            .chars()
            .enumerate()
            .map(|(i, _)| {
                let n = i as usize;
                let rem = haystack[n..].len();
                let e = i + (rem.min(nlen));
                (i as i32, haystack[n..e].to_string())
            })
            .collect();

        let result =
            if let Some((i, _)) = sset.iter().find(|(_, p)| *p == needle) {
                *i
            } else {
                -1
            };

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
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        let result = Solution::str_str(haystack, needle);
        assert_eq!(0, result);
    }

    #[test]
    fn test_example_2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();
        let result = Solution::str_str(haystack, needle);
        assert_eq!(-1, result);
    }
}
