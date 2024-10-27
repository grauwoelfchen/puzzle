use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn fixme(n: i32) -> i32 {
        let start = Instant::now();

        let result = (0..n).fold(0, |a, b| a ^ b);

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
    fn test_fixme() {
        let n = 2;
        let result = Solution::fixme(n);
        assert_eq!(1, result);
    }
}
