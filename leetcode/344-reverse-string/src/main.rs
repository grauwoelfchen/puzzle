#[allow(dead_code)]
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // s.reverse();
        let len = s.len();
        for i in 0..(len / 2) {
            s.swap(i, (len - i) - 1);
        }
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
        let mut s: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], s);
    }

    #[test]
    fn test_example_2() {
        let mut s: Vec<char> = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(vec!['h', 'a', 'n', 'n', 'a', 'H'], s);
    }
}
