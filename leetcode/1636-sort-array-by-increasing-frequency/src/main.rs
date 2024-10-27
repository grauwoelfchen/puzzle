use std::collections::HashMap;
use std::cmp::Ordering;

#[allow(dead_code)]
impl Solution {
    // Idea
    // 1. Use HashMap, count the occurences and sort
    // 2. ?
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for n in &nums {
            let v = map.entry(*n).or_insert(0);
            *v += 1;
        }

        let mut nums = nums;
        nums.sort_by(|a, b| {
            let o = map[a].cmp(&map[b]);
            if o == Ordering::Equal {
                b.cmp(&a)
            } else {
                o
            }
        });
        nums
    }
}

struct Solution;

fn main() {
    unimplemented!();
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums: Vec<i32> = vec![1, 1, 2, 2, 2, 3];
        let result = Solution::frequency_sort(nums);
        assert_eq!(vec![3, 1, 1, 2, 2, 2], result);
    }

    #[test]
    fn test_example_2() {
        let nums: Vec<i32> = vec![2, 3, 1, 3, 2];
        let result = Solution::frequency_sort(nums);
        assert_eq!(vec![1, 3, 3, 2, 2], result);
    }

    #[test]
    fn test_example_3() {
        let nums: Vec<i32> = vec![-1, 1, -6, 4, 5, -6, 1, 4, 1];
        let result = Solution::frequency_sort(nums);
        assert_eq!(vec![5, -1, 4, 4, -6, -6, 1, 1, 1], result);
    }
}
