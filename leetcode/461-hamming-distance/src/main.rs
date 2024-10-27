#[allow(dead_code)]
impl Solution {
    fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut result: i32 = 0;

        let long: Vec<char> = format!("{:b}", x.max(y)).chars().collect();
        let short: Vec<char> = format!("{:0>w$b}", x.min(y), w = long.len())
            .chars()
            .collect();

        for i in 0..long.len() {
            if i < short.len() {
                if short[i] != long[i] {
                    result += 1;
                }
            } else if '0' != long[i] {
                result += 1;
            }
        }
        result
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
        let result = Solution::hamming_distance(1, 4);
        assert_eq!(2, result);
    }

    #[test]
    fn test_example_2() {
        let result = Solution::hamming_distance(3, 1);
        assert_eq!(1, result);
    }
}
