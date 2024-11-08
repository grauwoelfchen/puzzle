use std::time::Instant;

#[allow(dead_code)]
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let start = Instant::now();

        // alphabet => count (occ)
        let mut freq = [i32::MAX; 26];

        for word in words {
            let mut occ = [0; 26];
            for b in word.bytes() {
                let i = (b - b'a') as usize;
                occ[i] += 1;
            }

            for n in 0..26 {
                freq[n] = i32::min(freq[n], occ[n]);
            }
        }

        let mut result = vec![];
        for (n, o) in freq.iter().enumerate() {
            for _ in 0..*o {
                let c = String::from((n as u8 + b'a') as char);
                result.push(c);
            }
        }

        /*
        // "cool" => [('c', 1), ('o', 2), ('o', 1), ('l' 1)]
        let pairs_list: Vec<Vec<(char, i32)>> = words
            .iter()
            .map(|w| {
                let chars: Vec<char> = w.chars().collect();
                let mut uniq = chars.clone();
                uniq.sort();
                uniq.dedup();

                let pairs: Vec<(char, i32)> = uniq
                    .iter()
                    .map(|c| {
                        (c, chars.iter().filter(|r| *r == c).count() as i32)
                    })
                    .map(|(s, n)| (1..=n).map(move |m| (*s, m)))
                    .flatten()
                    .collect();
                pairs
            })
            .collect();

        let least = pairs_list
            .iter()
            .min_by(|a, b| a.len().cmp(&b.len()))
            .unwrap();

        let result: Vec<String> = least
            .iter()
            .filter(|p| {
                dbg!(&p);
                pairs_list.iter().map(|pairs| pairs.contains(&p)).all(|b| b)
            })
            .map(|(s, _)| s.to_string())
            .collect();
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

    fn to_string(list: Vec<&str>) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_example_1() {
        let words = to_string(vec!["bella", "label", "roller"]);
        let mut result = Solution::common_chars(words);
        result.sort();
        assert_eq!(to_string(vec!["e", "l", "l"]), result);
    }

    #[test]
    fn test_example_2() {
        let words = to_string(vec!["cool", "lock", "cook"]);
        let mut result = Solution::common_chars(words);
        result.sort();
        assert_eq!(to_string(vec!["c", "o"]), result);
    }

    #[test]
    fn test_example_31() {
        let words = to_string(vec![
            "bbddabab", "cbcddbdd", "bbcadcab", "dabcacad", "cddcacbc",
            "ccbdbcba", "cbddaccc", "accdcdbb",
        ]);
        let mut result = Solution::common_chars(words);
        result.sort();
        assert_eq!(to_string(vec!["b", "d"]), result);
    }

    #[test]
    fn test_example_42() {
        let words = to_string(vec![
            "dbddaaac", "acaacdab", "acadbaaa", "dbaaabca", "ccddbabb",
            "bcaaadca", "cdcacbdb", "bdacccda",
        ]);
        let mut result = Solution::common_chars(words);
        result.sort();
        assert_eq!(to_string(vec!["a", "b", "c", "d"]), result);
    }
}
