use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let start = Instant::now();

        // No Hash{Map,Set}

        let m = nums.len() / 2;
        let mut dat = nums.clone();
        dat.sort();

        // majority element should pad more than half
        let result = dat[m];

        /*
        dat.dedup();
        dat.sort_by(a, b| b.partial_cmp(&a).unwrap());
        let result = *(dat.iter().find(|n| {
            nums.iter().filter(|x| x == n).count() > m
        }).unwrap());
        */

        /*
        // majority element always exists in the array
        dat.sort_by(|a, b| {
            let an = nums.iter().filter(|x| *x == a).count();
            let bn = nums.iter().filter(|x| *x == b).count();
            bn.partial_cmp(&an).unwrap()
        });
        let result = dat[0];
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
        let nums = vec![3, 2, 3];
        let result = Solution::majority_element(nums);
        assert_eq!(3, result);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let result = Solution::majority_element(nums);
        assert_eq!(2, result);
    }
}
