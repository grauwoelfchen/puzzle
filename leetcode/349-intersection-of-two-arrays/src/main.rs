use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    // A. use HashSet
    // B. get shorter one, look elements in another by using bin search
    #[rustfmt::skip]
    pub fn intersection(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    ) -> Vec<i32> {
        /*
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();

        set1
          .intersection(&set2)
          .into_iter()
          .cloned()
          .collect::<Vec<_>>()
        */

        let mut long = nums1.clone().max(nums2.clone());
        let short = nums1.clone().min(nums2.clone());

        long.sort();

        let mut result: HashSet<i32> = HashSet::new();
        for t in short {
            if let Ok(j) = long.binary_search(&t) {
                result.insert(long[j]);
            }
        }
        result.into_iter().collect()
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
    use std::collections::HashSet;
    use std::hash::Hash;

    use super::*;

    #[allow(dead_code)]
    fn eq_in_any_order<T>(a: &[T], b: &[T]) -> bool
    where
        T: Eq + Hash,
    {
        if a.len() != b.len() {
            return false;
        }

        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        a == b
    }

    #[macro_export]
    macro_rules! assert_eq_in_any_order {
        ($a:expr, $b:expr) => {
            assert!(eq_in_any_order(&$a, &$b))
        };
    }

    #[test]
    fn test_example_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];

        let result = Solution::intersection(nums1, nums2);
        assert_eq_in_any_order!(vec![2], result);
    }

    #[test]
    fn test_example_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];

        let result = Solution::intersection(nums1, nums2);
        assert_eq_in_any_order!(vec![9, 4], result);
    }

    #[test]
    fn test_example_21() {
        let nums1 = vec![2, 1];
        let nums2 = vec![1, 2];

        let result = Solution::intersection(nums1, nums2);
        assert_eq_in_any_order!(vec![1, 2], result);
    }

    #[test]
    fn test_example_19() {
        let nums1 = vec![3, 1, 2];
        let nums2 = vec![1, 3];

        let result = Solution::intersection(nums1, nums2);
        assert_eq_in_any_order!(vec![3, 1], result);
    }
}
