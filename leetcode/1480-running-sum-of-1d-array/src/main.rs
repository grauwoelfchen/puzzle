use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let start = Instant::now();

        let result = nums
            .iter()
            .enumerate()
            .map(|(i, _)| nums[0..=i].iter().sum())
            .collect();

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
        let nums = vec![1, 2, 3, 4];
        let result = Solution::running_sum(nums);
        assert_eq!(vec![1, 3, 6, 10], result);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = Solution::running_sum(nums);
        assert_eq!(vec![1, 2, 3, 4, 5], result);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![3, 1, 2, 10, 1];
        let result = Solution::running_sum(nums);
        assert_eq!(vec![3, 4, 6, 16, 17], result);
    }
}
