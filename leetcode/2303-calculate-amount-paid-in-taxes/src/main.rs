#[allow(dead_code)]
impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut result: f64 = 0.0;

        if income == 0 {
            return result;
        }

        let mut a = 0;
        for b in brackets {
            let u = b[0];
            let t = b[1];

            let i = income - a;
            if i < 0 {
                break;
            }
            let n = u - a;
            result += (n.min(i) as f64) * (t as f64 / 100.0);
            a += n;
        }
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
        let brackets = vec![vec![3, 50], vec![7, 10], vec![12, 25]];
        let income = 10;
        let result = Solution::calculate_tax(brackets, income);
        assert_eq!(2.65000, result);
    }

    #[test]
    fn test_example_2() {
        let brackets = vec![vec![1, 0], vec![4, 25], vec![5, 50]];
        let income = 2;
        let result = Solution::calculate_tax(brackets, income);
        assert_eq!(0.25000, result);
    }

    #[test]
    fn test_example_3() {
        let brackets = vec![vec![2, 50]];
        let income = 0;
        let result = Solution::calculate_tax(brackets, income);
        assert_eq!(0.00000, result);
    }
}
