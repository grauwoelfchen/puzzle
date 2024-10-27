// use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // A
        // nums.dedup();
        // nums.len() as i32

        // B
        // let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());
        // let mut dup: usize = 0;

        // for i in 0..nums.len() {
        //     let j = i as usize - dup;
        //     let n = nums[j];
        //     if set.contains(&n) {
        //         nums.remove(j);
        //         dup += 1;
        //     } else {
        //         set.insert(n);
        //     }
        // }
        // nums.len() as i32

        // C
        let mut prv = 0;

        for i in 1..nums.len() {
            if nums[prv] != nums[i] {
                prv += 1;
                nums[prv] = nums[i];
            }
        }
        (prv + 1) as i32
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
        let mut nums: Vec<i32> = vec![1, 1, 2];
        let expected_nums: Vec<i32> = vec![1, 2];

        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(2, k);

        for i in 0..(k as usize) {
            assert_eq!(expected_nums[i], nums[i]);
        }
    }

    #[test]
    fn test_example_2() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected_nums: Vec<i32> = vec![0, 1, 2, 3, 4];

        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(5, k);

        for i in 0..(k as usize) {
            assert_eq!(expected_nums[i], nums[i]);
        }
    }

    #[test]
    fn test_example_188() {
        let mut nums: Vec<i32> = vec![1, 1];
        let expected_nums: Vec<i32> = vec![1];

        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(1, k);

        for i in 0..(k as usize) {
            assert_eq!(expected_nums[i], nums[i]);
        }
    }
}
