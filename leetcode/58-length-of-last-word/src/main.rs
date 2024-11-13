use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    // 1 <= s.length <= 10^4
    pub fn length_of_last_word(s: String) -> i32 {
        let start = Instant::now();

        // eg. 3.03ms, 1.95ms, 1.70ms
        let word = s.trim().split(' ').rev().take(1).collect::<String>();
        let result = word.len() as i32;

        // eg. 2.82ms, 2.74ms, 981.00ns
        // let result = s.trim().split(' ').last().unwrap_or("").len() as i32;

        // eg. 1.91ms, 1.43ms, 1.63ms
        /*
        let result = s.trim().chars().fold(0, |mut acc, c| {
            if c == ' ' {
                acc = 0;
            } else {
                acc += 1;
            }
            acc
        });
        */

        // eg. 1.41ms, 1.31ms, 1.14ms
        /*
        let mut n = 0;
        for c in s.trim().chars().rev() {
            if c == ' ' {
                break;
            } else {
                n += 1;
            }
        }
        let result = n;
        */

        // eg. 1.40ms, 2.34ms, 1.49ms
        /*
        let mut n = 0;
        for c in s.trim().chars() {
            if c == ' ' {
                n = 0;
            } else {
                n += 1;
            }
        }
        let result = n;
        */

        // eg. 3.03ms, 2.67ms, 1.83ms
        /* ref
        let chars: Vec<char> = s.chars().collect();

        let mut p = s.len() - 1;
        while chars[p] == ' ' {
            p -= 1;
        }
        let mut result = 0;
        while chars[p] != ' ' {
            p -= 1;
            result += 1;
        }
        */

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
        let s = "Hello World".to_string();
        let result = Solution::length_of_last_word(s);
        assert_eq!(5, result);
    }

    #[test]
    fn test_example_2() {
        let s = "   fly me   to   the moon  ".to_string();
        let result = Solution::length_of_last_word(s);
        assert_eq!(4, result);
    }

    #[test]
    fn test_example_3() {
        let s = "luffy is still joyboy".to_string();
        let result = Solution::length_of_last_word(s);
        assert_eq!(6, result);
    }
}
