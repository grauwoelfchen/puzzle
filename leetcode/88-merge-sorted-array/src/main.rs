#[allow(dead_code)]
impl Solution {
    fn binary_search(nums: &[i32], e: &i32) -> Result<usize, usize> {
        if nums.is_empty() {
            return Err(0);
        }
        let mut min: i32 = 0;
        let mut max: i32 = nums.len() as i32 - 1;

        while min <= max {
            let mid = (min + max) / 2;
            let val = nums[mid as usize];
            if val == *e {
                return Ok(mid as usize);
            }
            if val > *e {
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }
        Err((max + 1) as usize)
    }

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut mm = m;

        for i in &nums2[0..(n as usize)] {
            // match nums1[0..(mm as usize)].binary_search(&i) {
            match Self::binary_search(&nums1[0..(mm as usize)], i) {
                Ok(j) => nums1.insert(j + 1, *i),
                Err(k) => nums1.insert(k, *i),
            }
            nums1.pop();
            mm += 1;
        }
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
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
    }

    #[test]
    fn test_example_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(vec![1], nums1);
    }

    #[test]
    fn test_example_3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(vec![1], nums1);
    }
}
