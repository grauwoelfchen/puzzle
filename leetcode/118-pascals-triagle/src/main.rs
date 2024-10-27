#[allow(dead_code)]
impl Solution {
    // 1 <= num_rows <= 30
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        #[rustfmt::skip]
        let base: Vec<Vec<i32>> = vec![
            vec![1],
            vec![1, 1]
        ];

        if num_rows < 3 {
            return base[0..(num_rows as usize)].to_vec();
        }

        // []             ->  1
        // []             ->  2
        // [2]            ->  4
        // [3, 3]         ->  8
        // [4, 6, 4]      -> 16
        // [5, 10, 10, 5] -> 32
        // [6,         6] -> 64

        let mut triangle = base;
        for n in 3..=num_rows {
            let prev = &triangle[(n - 2) as usize];

            let mut line: Vec<i32> = vec![];
            line.append(&mut vec![1]);
            line.append(&mut prev.windows(2).map(|v| v[0] + v[1]).collect());
            line.append(&mut vec![1]);

            // let mut line: Vec<i32> = vec![1];
            // let mut j = n - 2;
            // while j > 0 {
            //     let x = prev[(j - 1) as usize] + prev[j as usize];
            //     line.push(x);
            //     j -= 1;
            // }
            // line.push(1);
            triangle.push(line);
        }
        triangle
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
        let num_rows = 5;
        let result = Solution::generate(num_rows);
        assert_eq!(
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ],
            result
        );
    }

    #[test]
    fn test_example_2() {
        let num_rows = 1;
        let result = Solution::generate(num_rows);
        assert_eq!(vec![vec![1],], result);
    }
}
