use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    // 1 <= nums.length <= 10^5
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let start = Instant::now();

        // ex. 3.96µs, 3.48µs
        /*
        let result: usize = nums
            .split(|n| n == &0)
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len();
        */

        // ex. 491.00ns, 480.00ns
        let mut m = 0;
        let mut c = 0;
        for n in nums {
            if n == 0 {
                c = 0;
            } else {
                c += 1;
            }
            if m < c {
                m = c;
            }
        }
        let result = m;

        println!("{:.2?}", start.elapsed());
        result as i32
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
        let nums = vec![1, 1, 0, 1, 1, 1];
        let result = Solution::find_max_consecutive_ones(nums);
        assert_eq!(3, result);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        let result = Solution::find_max_consecutive_ones(nums);
        assert_eq!(2, result);
    }
}
