use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn moving_modest_minimum(list: Vec<i32>) -> Vec<i32> {
        let start = Instant::now();

        let mut sorted = list.clone();
        sorted.sort();

        // main
        let result = match (sorted.get(0), sorted.get(1)) {
            (Some(m), Some(n)) => list
                .iter()
                .map(|v| if v >= n { m } else { n })
                .cloned()
                .collect::<Vec<i32>>(),
            _ => unreachable!(),
        };

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
        let list = vec![4, 3, 2, 5];
        let result = Solution::moving_modest_minimum(list);
        assert_eq!(vec![2, 2, 3, 2], result);
    }

    #[test]
    fn test_example_2() {
        let list = vec![4, 2, 2, 5];
        let result = Solution::moving_modest_minimum(list);
        assert_eq!(vec![2, 2, 2, 2], result);
    }

    #[test]
    fn test_example_3() {
        let list = vec![6, 3, 5, 5, 8];
        let result = Solution::moving_modest_minimum(list);
        assert_eq!(vec![3, 5, 3, 3, 3], result);
    }

    #[test]
    fn test_example_4() {
        let list = vec![7, 1];
        let result = Solution::moving_modest_minimum(list);
        assert_eq!(vec![1, 7], result);
    }

    #[test]
    fn test_example_5() {
        let list = vec![9, 9];
        let result = Solution::moving_modest_minimum(list);
        assert_eq!(vec![9, 9], result);
    }

    #[test]
    fn test_example_6() {
        let list = vec![9, 8, 9];
        let result = Solution::moving_modest_minimum(list);
        assert_eq!(vec![8, 9, 8], result);
    }
}
