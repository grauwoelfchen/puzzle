// use std::time::Instant;
// use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let t: usize = n as usize - 1;
        if trust.len() < t {
            return -1;
        }

        let mut score = vec![0; n as usize + 1];
        for r in &trust {
            score[r[0] as usize] -= 1;
            score[r[1] as usize] += 1;
        }

        let t = n - 1;
        for i in 1..=n {
            if score[i as usize] == t {
                return i;
            }
        }
        return -1;

        /*
        let start = Instant::now();

        if n == 1 && trust.is_empty() {
            return 1;
        }

        // eg. 17.80µs, 12.88µs
        let mut data: HashMap<i32, i32> = HashMap::new();
        let mut people: Vec<i32> = vec![];

        for p in trust {
            let w = p[0];
            let t = p[1];

            if !people.contains(&w) {
                people.push(w);
            }

            // count
            if let Some(v) = data.get_mut(&t) {
                *v += 1;
            } else {
                data.insert(t, 1);
            }
        }

        // everybody trust judge
        let t = n - 1;
        let (mut result, _) =
            data.into_iter().find(|(_, v)| *v == t).unwrap_or((-1, 0));

        // judge trusts nobody
        if people.contains(&result) {
            result = -1;
        }

        println!("{:.2?}", start.elapsed());
        result
        */
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
        let n = 2;
        let trust = vec![vec![1, 2]];
        let result = Solution::find_judge(n, trust);
        assert_eq!(2, result);
    }

    #[test]
    fn test_example_2() {
        let n = 3;
        let trust = vec![vec![1, 3], vec![2, 3]];
        let result = Solution::find_judge(n, trust);
        assert_eq!(3, result);
    }

    #[test]
    fn test_example_3() {
        let n = 3;
        let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
        let result = Solution::find_judge(n, trust);
        assert_eq!(-1, result);
    }

    #[test]
    fn test_example_4() {
        let n = 4;
        let trust =
            vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]];
        let result = Solution::find_judge(n, trust);
        assert_eq!(3, result);
    }

    #[test]
    fn test_example_91() {
        let n = 1;
        let trust = vec![];
        let result = Solution::find_judge(n, trust);
        assert_eq!(1, result);
    }
}
