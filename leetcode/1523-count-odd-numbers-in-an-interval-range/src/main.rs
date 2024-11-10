use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    // 0 <= low <= high <= 10^9
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let start = Instant::now();

        // let result = (low..=high).filter(|n| n % 2 != 0).count() as i32;
        let result = (high + 1) / 2 - low / 2;

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
        let low = 3;
        let high = 7;
        let result = Solution::count_odds(low, high);
        assert_eq!(3, result);
    }

    #[test]
    fn test_example_2() {
        let low = 8;
        let high = 10;
        let result = Solution::count_odds(low, high);
        assert_eq!(1, result);
    }
}
