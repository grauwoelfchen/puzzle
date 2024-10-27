use std::collections::HashSet;
// use std::iter::FromIterator;

#[allow(dead_code)]
impl Solution {
    // 1. HashSet
    // 2. Sort + checking prev value?
    // 3. Get an element + binary_search?
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // nums.len() != HashSet::<i32>::from_iter(nums).len()
  
        // eg. [1, 2, 3, 4, -10, 9, 8, -2, -3, 4, 1]
        /* this is a bit slow
        let mut nums = nums;
        nums.sort();
        */

        let mut set: HashSet<&i32> = HashSet::new();
        // !nums.iter().all(|n| set.insert(n))

        for n in &nums {
            if set.contains(n) {
                return true;
            }
            set.insert(n);
        }
        set.len() != nums.len()
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
        let nums = vec![1, 2, 3, 1];
        let result = Solution::contains_duplicate(nums);
        assert_eq!(true, result);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::contains_duplicate(nums);
        assert_eq!(false, result);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result = Solution::contains_duplicate(nums);
        assert_eq!(true, result);
    }
}
