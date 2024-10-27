#[allow(dead_code)]
impl Solution {
    // 2 <= n <= 58
    fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }

        // 10 / 3 -> 3 + 1 ===> 3 * 3 * 4

        // 3s
        let n3s = (n / 3) as u32;
        let n3r = n % 3; // 0, 1 or 2

        match n3r {
            0 => 3_i32.pow(n3s),
            1 => 3_i32.pow(n3s - 1) * 4,
            _ => 3_i32.pow(n3s) * 2,
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
        let n = 2;
        let result = Solution::integer_break(n);
        assert_eq!(1, result);
    }

    #[test]
    fn test_example_2() {
        let n = 10;
        let result = Solution::integer_break(n);
        assert_eq!(36, result);
    }
}
