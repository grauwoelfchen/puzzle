use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    // 1 <= nums.length <= 10^5
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let start = Instant::now();

        // eg. 8.94µs, 5.54µs
        /*
        let lens: Vec<i32> =
            nums.split(|n| n == &0).map(|a| a.len() as i32).collect();
        let result = lens
            .iter()
            .enumerate()
            .map(|(i, n)| {
                if i > 0 {
                    let v = lens[i - 1] + 1 + n;
                    if i < lens.len() - 1 {
                        v.max(n + 1 + lens[i + 1])
                    } else {
                        v
                    }
                } else {
                    *n
                }
            })
            .max()
            .unwrap_or(0);
        */

        // eg. 6.01µs, 3.71µs
        let len = nums.len() as i32;
        if len <= 1 {
            return len;
        }
        if !nums.contains(&0) {
            return len;
        }
        let result: i32 = nums
            .split(|n| n == &0)
            .map(|a| a.len() as i32)
            .collect::<Vec<i32>>()
            .windows(2)
            .map(|n| n[0] + 1 + n[1])
            .max()
            .unwrap();

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
        let nums = vec![1, 0, 1, 1, 0];
        let result = Solution::find_max_consecutive_ones(nums);
        assert_eq!(4, result);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        let result = Solution::find_max_consecutive_ones(nums);
        assert_eq!(4, result);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1];
        let result = Solution::find_max_consecutive_ones(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_example_33() {
        let nums = vec![1, 1, 0, 1];
        let result = Solution::find_max_consecutive_ones(nums);
        assert_eq!(4, result);
    }

    #[test]
    fn test_example_39() {
        let nums = vec![1, 1];
        let result = Solution::find_max_consecutive_ones(nums);
        assert_eq!(2, result);
    }
}
