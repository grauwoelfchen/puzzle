#[allow(dead_code)]
impl Solution {
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for (i, v) in nums.clone().iter().enumerate() {
            if *v == val {
                nums.remove(i - k);
                k += 1;
            }
        }
        nums.len() as i32
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
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let result = Solution::remove_element(&mut nums, val);
        nums.sort();

        assert_eq!(2, result);
        assert_eq!([2, 2], nums[..]);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let result = Solution::remove_element(&mut nums, val);
        nums.sort();

        assert_eq!(5, result);
        assert_eq!([0, 0, 1, 3, 4], nums[..]);
    }
}
