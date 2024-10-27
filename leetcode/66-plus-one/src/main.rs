use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let start = Instant::now();

        let mut result = vec![];

        let mut num: i32 = 1;
        for n in digits.iter().rev() {
            let mut e = n + num;
            if e >= 10 {
                e = 0;
                num = 1;
            } else {
                num = 0;
            }
            result.push(e);
        }
        if num == 1 {
            result.push(1);
        }

        println!("{:.2?}", start.elapsed());

        // result.iter().map(|n| *n).rev().collect()
        // result.into_iter().rev().collect()
        result.reverse();
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
        let digits = vec![1, 2, 3];
        let result = Solution::plus_one(digits);
        assert_eq!(vec![1, 2, 4], result);
    }

    #[test]
    fn test_example_2() {
        let digits = vec![4, 3, 2, 1];
        let result = Solution::plus_one(digits);
        assert_eq!(vec![4, 3, 2, 2], result);
    }

    #[test]
    fn test_example_3() {
        let digits = vec![9];
        let result = Solution::plus_one(digits);
        assert_eq!(vec![1, 0], result);
    }

    #[test]
    fn test_example_58() {
        let digits = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let result = Solution::plus_one(digits);
        assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1], result);
    }
}
