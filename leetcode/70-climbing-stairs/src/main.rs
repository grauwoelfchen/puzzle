use std::time::Instant;
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    fn calc(n: i32, mut memo: HashMap<i32, i32>) -> i32 {
        let v = memo.get(&n);
        if v.is_some() {
            *v.unwrap()
        } else {
            let ib = n - 2;
            let b = Self::calc(ib, memo.clone());
            memo.insert(ib, b);
            // println!("b: {}", b);

            let ia = n - 1;
            let a = Self::calc(ia, memo.clone());
            memo.insert(ia, a);
            // println!("a: {}", a);

            let result = a + b;
            memo.insert(n, result);
            result
        }
    }

    pub fn climb_stairs(n: i32) -> i32 {
        let start = Instant::now();

        let mut memo: HashMap<i32, i32> = HashMap::new();
        memo.insert(0, 0);
        memo.insert(1, 1);
        memo.insert(2, 2);
        memo.insert(3, 3);
        let result = Self::calc(n, memo);

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

    // 1 <= n <= 45

    #[test]
    fn test_example_1() {
        let n = 2;
        let result = Solution::climb_stairs(n);
        assert_eq!(2, result);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let result = Solution::climb_stairs(n);
        assert_eq!(3, result);
    }

    #[test]
    fn test_example_3() {
        let n = 4;
        let result = Solution::climb_stairs(n);
        assert_eq!(5, result);
    }

    // 1 1 1 1 1
    // 2 2 1
    // 2 1 2
    // 1 1 2
    // 1 1 1 2
    // 1 1 2 1
    // 1 2 1 1
    // 2 1 1 1
    #[test]
    fn test_example_4() {
        let n = 5;
        let result = Solution::climb_stairs(n);
        assert_eq!(8, result);
    }

    #[test]
    fn test_example_5() {
        let n = 6;
        let result = Solution::climb_stairs(n);
        assert_eq!(13, result);
    }

    #[test]
    fn test_example_6() {
        let n = 45;
        let result = Solution::climb_stairs(n);
        assert_eq!(1836311903, result);
    }
}
