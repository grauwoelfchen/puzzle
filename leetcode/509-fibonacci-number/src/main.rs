#[allow(dead_code)]
impl Solution {
    // 1. Calculate recursively
    // 2. create possible a Vec<i32>, return arr[n - 1]
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        // Self::fib(n - 1) + Self::fib(n - 2)

        let mut arr: Vec<i32> = vec![];
        arr.push(0);
        arr.push(1);

        for i in 2..(n + 1) {
            arr.push(arr[(i - 1) as usize] + arr[(i - 2) as usize]);
        }
        arr[n as usize]

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
        let n = 2;
        let result = Solution::fib(n);
        assert_eq!(1, result);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let result = Solution::fib(n);
        assert_eq!(2, result);
    }

    #[test]
    fn test_example_3() {
        let n = 4;
        let result = Solution::fib(n);
        assert_eq!(3, result);
    }
}
