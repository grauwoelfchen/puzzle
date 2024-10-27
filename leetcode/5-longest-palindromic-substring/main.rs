#[allow(dead_code)]
impl Solution {
    /*
    fn is_palindrome(s: &String) -> bool {
        let len = s.len();
        if len == 1 {
            return true;
        }
        // digits and english letters only
        let chars = s.as_bytes();

        if len == 2 && (chars[0] == chars[1]) {
            return true;
        }

        let d = match len % 2 {
            0 => 0,
            _ => 1,
        };
        for n in 0..(len - 1) {
            if n >= (len - d) / 2 {
                break;
            }
            let bn = (len - 1) - n;
            if chars[n] != chars[bn] {
                return false;
            }
        }
        true
    }
    */

    fn is_palindrome(s: &[u8]) -> bool {
        // abc <-> cba
        s.iter().zip(s.iter().rev()).all(|(a, b)| a == b)
    }

    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        for n in (1..=s.len()).rev() {
            if let Some(v) = bytes.windows(n).find(|sub| Self::is_palindrome(sub)) {
                return String::from_utf8(v.to_vec()).unwrap();
            }
        }
        "".to_string()

        /*
        let mut res = "".to_string();
        for n in 0..s.len() {
            let mut tmp = "".to_string();
            for c in s[n..].chars() {
                tmp.push(c);
                if Self::is_palindrome(&tmp) && res.len() < tmp.len() {
                    res = tmp.clone();
                }
            }
        }
        res
        */
    }
}

// -----
struct Solution;

fn main() {
    unimplemented!();
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;

    // abbba
    // abcba
    // aabbaa
    // aaaaaa

    #[test]
    fn test_longest_palindrome_1() {
        let s = "babad".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bab");
    }

    #[test]
    fn test_longest_palindrome_2() {
        let s = "cbbd".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bb");
    }
}
