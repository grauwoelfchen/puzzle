use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let start = Instant::now();

        if nums.len() == 1 {
            return nums[0];
        }
        let (_, result) = nums
            .iter()
            .enumerate()
            .find(|(i, n)| {
                let k = *i as usize;
                let rest = [&nums[0..k], &nums[(k + 1)..]].concat();
                !rest.contains(n)
            })
            .unwrap();

        println!("{:.2?}", start.elapsed());
        *result
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
        let nums = vec![2, 2, 1];
        let result = Solution::single_number(nums);
        assert_eq!(1, result);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![4, 1, 2, 1, 2];
        let result = Solution::single_number(nums);
        assert_eq!(4, result);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1];
        let result = Solution::single_number(nums);
        assert_eq!(1, result);
    }
}
