use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    // 1 <= words.length <= 20
    // 1 <= words[i].length <= 100
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let start = Instant::now();

        let rows: [Vec<char>; 3] = [
            vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
            vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
            vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
        ];

        // eg. 5.20ms, 2.59ms, 2.48ms
        let mut result = vec![];
        for w in words {
            let t = w.to_lowercase();
            for r in &rows {
                if t.chars().all(|c| r.contains(&c)) {
                    result.push(w);
                    break;
                }
            }
        }

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

    fn to_s(words: Vec<&str>) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_example_1() {
        let words = to_s(vec!["Hello", "Alaska", "Dad", "Peace"]);
        let result = Solution::find_words(words);
        assert_eq!(to_s(vec!["Alaska", "Dad"]), result);
    }

    #[test]
    fn test_example_2() {
        let words = to_s(vec!["omk"]);
        let result = Solution::find_words(words);
        assert_eq!(to_s(vec![]), result);
    }

    #[test]
    fn test_example_3() {
        let words = to_s(vec!["adsdf", "sfd"]);
        let result = Solution::find_words(words);
        assert_eq!(to_s(vec!["adsdf", "sfd"]), result);
    }
}
