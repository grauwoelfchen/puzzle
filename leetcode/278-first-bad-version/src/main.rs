use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    // 1 <= bad <= n <= 2^31 - 1

    pub fn new(bad: i32) -> Self {
        Self { bad }
    }

    // Apparently, it seems that the signature of this API needs to be
    // isBadVesion() on Leetcode (without snake_case)
    #[allow(non_snake_case)]
    fn isBadVersion(&self, n: i32) -> bool {
        self.is_bad_version(n)
    }

    fn is_bad_version(&self, n: i32) -> bool {
        n >= self.bad
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let start = Instant::now();

        let mut min: i32 = 0;
        let mut max = n;

        while min < max {
            let mid = ((min as i64 + max as i64) / 2) as i32;
            if self.isBadVersion(mid) {
                max = mid;
            } else {
                min = mid + 1;
            }
        }
        let result = min;

        println!("{:.2?}", start.elapsed());
        result
    }
}

pub struct Solution {
    bad: i32,
}

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
        let n = 5;
        let bad = 4;
        let solution = Solution::new(bad);
        let result = solution.first_bad_version(n);
        assert_eq!(4, result);
    }

    #[test]
    fn test_example_2() {
        let n = 1;
        let bad = 1;
        let solution = Solution::new(bad);
        let result = solution.first_bad_version(n);
        assert_eq!(1, result);
    }

    #[test]
    fn test_example_9() {
        let n = 3;
        let bad = 1;
        let solution = Solution::new(bad);
        let result = solution.first_bad_version(n);
        assert_eq!(1, result);
    }

    #[test]
    fn test_example_11() {
        let n = 2126753390;
        let bad = 1702766719;
        let solution = Solution::new(bad);
        let result = solution.first_bad_version(n);
        assert_eq!(1702766719, result);
    }
}
