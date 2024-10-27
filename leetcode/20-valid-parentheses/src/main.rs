#[allow(dead_code)]
impl Solution {
    fn to_open(c: &char) -> Option<char> {
        match c {
            ')' => Some('('),
            ']' => Some('['),
            '}' => Some('{'),
            _ => None,
        }
    }

    pub fn is_valid(s: String) -> bool {
        if s.len() & 1 == 1 {
            return false;
        }

        let mut m: Vec<char> = Vec::with_capacity(s.len()); // stack

        for c in s.chars() {
            if let Some(o) = Self::to_open(&c) {
                match m.pop() {
                    Some(p) if o == p => (),
                    _ => return false,
                }
            } else {
                m.push(c);
            }
        }
        m.len() == 0
    }
}

struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        let s = "".to_string();
        assert_eq!(Solution::is_valid(s), true);

        let s = "()".to_string();
        assert_eq!(Solution::is_valid(s), true);

        let s = "()[]{}".to_string();
        assert_eq!(Solution::is_valid(s), true);

        let s = "(]".to_string();
        assert_eq!(Solution::is_valid(s), false);
    }
}
