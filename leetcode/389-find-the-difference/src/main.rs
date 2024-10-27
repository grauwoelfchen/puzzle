#[allow(dead_code)]
impl Solution {
    // 1. sort both strings, and compare chars one by one
    // 2. sort only t, and find the char by using binary search
    // 3. change chars to the number and get the diff of two sums
    pub fn find_the_difference(s: String, t: String) -> char {
        let na = s.as_bytes().iter().fold(0, |acc, b| {
            acc + *b as u32
        });
        let nt = t.as_bytes().iter().fold(0, |acc, b| {
            acc + *b as u32
        });
        char::from_u32(nt -  na).unwrap()
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
        let s = "abcd".to_string();
        let t = "abcde".to_string();
        let result = Solution::find_the_difference(s, t);
        assert_eq!('e', result);
    }

    #[test]
    fn test_example_2() {
        let s = "".to_string();
        let t = "y".to_string();
        let result = Solution::find_the_difference(s, t);
        assert_eq!('y', result);
    }
}
