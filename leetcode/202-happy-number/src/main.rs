// use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    /* with HashSet
    fn is_happy(n: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut t: i32 = n;

        loop {
            if seen.contains(&t) {
                return false;
            }
            seen.insert(t);

            let s = t.to_string();
            let sum = s.chars().fold(0, |mut acc, x| {
                if let Some(i) = x.to_digit(10) {
                    acc += i * i;
                }
                acc
            });
            if sum == 1 {
                return true;
            }
            t = sum as i32;
        }
    }
    */

    fn process(t: i32) -> i32 {
        let s = t.to_string();
        let sum = s.chars().fold(0, |mut acc, x| {
            if let Some(i) = x.to_digit(10) {
                acc += i * i;
            }
            acc
        });
        sum as i32
    }

    fn is_happy(n: i32) -> bool {
        let (mut slow, mut fast) = (n, n);
        loop {
            slow = Self::process(slow);
            fast = Self::process(Self::process(fast));
 
            if fast == 1 || slow == 1 {
                return true;
            }
            if slow == fast { // in a cycle
                break;
            }
        }
        false
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
        let n = 19;
        let result = Solution::is_happy(n);
        assert!(result);
    }

    #[test]
    fn test_example_2() {
        let n = 2;
        let result = Solution::is_happy(n);
        assert!(!result);
    }
}
