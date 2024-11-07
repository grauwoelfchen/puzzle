use std::time::Instant;
// use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let start = Instant::now();

        let m = nums.len();
        let result = nums.iter().enumerate().fold(0, |acc, (i, n)| {
            if !nums[i + 1..m].contains(&n) && !nums[0..i].contains(&n) {
                acc + n
            } else {
                acc
            }
        });

        /*
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            let mut v = 1;
            if let Some(c) = map.get_mut(&n) {
                v += *c;
            }
            map.insert(n, v);
        }

        let result = map
            .iter()
            .filter(|(_, c)| **c == 1)
            .fold(0, |acc, (n, _)| acc + n);
        */

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
        let nums = vec![1, 2, 3, 2];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(4, result);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1, 1, 1];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(0, result);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(15, result);
    }
}
