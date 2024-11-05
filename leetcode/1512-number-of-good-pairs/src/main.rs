use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let start = Instant::now();

        /*
        let len = nums.len();
        let result = nums.iter().enumerate().fold(0, |acc, (i, n)| {
            acc + ((i + 1)..len).filter(|j| nums[*j] == *n).count()
        });
        */
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result = 0;

        for n in nums {
            result += map.get(&n).unwrap_or(&0);
            if let Some(v) = map.get_mut(&n) {
                *v = *v + 1;
            } else {
                map.insert(n, 1);
            }
        }

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
        let nums = vec![1, 2, 3, 1, 1, 3];
        let result = Solution::num_identical_pairs(nums);
        assert_eq!(4, result);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1, 1, 1];
        let result = Solution::num_identical_pairs(nums);
        assert_eq!(6, result);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 3];
        let result = Solution::num_identical_pairs(nums);
        assert_eq!(0, result);
    }
}
