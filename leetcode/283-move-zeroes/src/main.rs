#[allow(dead_code)]
impl Solution {
    // 1. get index and remove
    // 2. count zeroes and filter
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        /*
        let binding = nums.clone();
        let indexes = binding
            .iter()
            .enumerate()
            .filter(|(_, n)| n == &&0)
            .map(|(i, _)| i);

        for i in indexes.rev() {
            let z = nums.remove(i);
            nums.push(z);
        }
        */

        // or drain_filter() (nightly)
        let mut i = 0;
        nums.retain(|&n| {
            let r = n != 0;
            if !r {
                // zero
                i += 1;
            }
            r
        });
        // or append()
        for _ in 0..i {
            nums.push(0);
        }

        // e.g. two pointers
        //
        // [0, 1,  0, 3, 12]
        // [0, 1,  0, 3, 12] fast 0 slow 0
        // [1, 0,  0, 3, 12] fast 1 slow 0 + 1
        // [1, 3,  0, 0, 12] fast 2 slow 1 + 1
        // [1, 3,  0, 0, 12] fast 3 slow 2
        // [1, 3, 12, 0,  0] fast 4 slow 2 + 1
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
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(vec![1, 3, 12, 0, 0], nums);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(vec![0], nums);
    }
}
