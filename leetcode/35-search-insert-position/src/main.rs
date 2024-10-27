use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let start = Instant::now();

        /*
        match nums.binary_search(&target) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
        */

        /*
        let mut nums = nums;
        nums.push(target);
        nums.sort();

        let result = if let Some((index, _)) =
            nums.iter().enumerate().find(|(_, n)| *n == &target)
        {
            index as i32
        } else {
            -1
        };

        println!("{:.2?}", start.elapsed());
        result
        */

        let mut lower: i32 = 0;
        let mut upper: i32 = nums.len() as i32;

        while lower < upper {
            let mid = (lower + upper) / 2;
            if nums[mid as usize] >= target {
                upper = mid;
            } else {
                lower = mid + 1;
            }
        }

        println!("{:.2?}", start.elapsed());
        lower
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
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let result = Solution::search_insert(nums, target);
        assert_eq!(2, result);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let result = Solution::search_insert(nums, target);
        assert_eq!(1, result);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let result = Solution::search_insert(nums, target);
        assert_eq!(4, result);
    }
}
