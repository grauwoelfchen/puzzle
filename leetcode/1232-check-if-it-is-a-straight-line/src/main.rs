#[allow(dead_code)]
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        // y = 5x + 2
        // [1, 7]  1/7  = 0.1428
        // [2, 12] 2/12 = 0.166
        // [3, 17] 3/17 = 1.764
        // [4, 22]
        // [5, 27]
        // [6, 32]
        //
        // [x, y] x/y
        // [1, 5] 1/5
        // [2,10] 2/10
        // [3,15] 3/15
        //
        // [x, y] x/y
        // [1, 1] 1
        // [2, 2] 1
        // [3, 3] 1
        //
        // [x, y] x/y
        // [2, 1]  2
        // [4, 2]  2
        // [6, 3]  2
        //
        //
        // [1, 2]
        // [2, 3]
        // [3, 4]
        // [4, 5],
        // [5, 6],
        // [6, 7],
        //
        // [2,4] 2
        // [2,5] 3
        // [2,8] 6
        //
        // 2x + 1
        // [0, 1]
        // [1, 3]
        // [-4,-7]
        // [5, 11]

        let a = coordinates
            .iter()
            .filter(|c| 2 * c[0] + 1 == c[1])
            .collect::<Vec<_>>();
        if coordinates.len() == a.len() {
            return true;
        }

        let mut x = coordinates.iter().map(|c| c[0]).collect::<Vec<_>>();
        x.dedup();
        if x.len() == 1 {
            return true;
        }

        let mut y = coordinates.iter().map(|c| c[1]).collect::<Vec<_>>();
        y.dedup();
        if y.len() == 1 {
            return true;
        }

        let v = coordinates
            .iter()
            .filter(|c| !(c[0] == 0 && c[1] == 0))
            .map(|c| {
                let m = (c[0] as f32).abs() / (c[1] as f32).abs();
                let d = c[0].abs() - c[1].abs();
                (m, d)
            })
            .collect::<Vec<(f32, i32)>>();

        let mut m = v.iter().map(|(m, _)| *m).collect::<Vec<f32>>();
        m.dedup();
        if m.len() == 1 {
            return true;
        }

        let mut d = v.iter().map(|(_, d)| *d).collect::<Vec<i32>>();
        d.dedup();
        if d.len() == 1 {
            return true;
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
        #[rustfmt::skip]
        let coordinates = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
        ];
        let result = Solution::check_straight_line(coordinates);
        assert!(result);
    }

    #[test]
    fn test_example_2() {
        #[rustfmt::skip]
        let coordinates = vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
        ];
        let result = Solution::check_straight_line(coordinates);
        assert!(!result);
    }

    #[test]
    fn test_input_47() {
        #[rustfmt::skip]
        let coordinates = vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, -1],
        ];
        let result = Solution::check_straight_line(coordinates);
        assert!(result);
    }

    #[test]
    fn test_input_46() {
        #[rustfmt::skip]
        let coordinates = vec![
            vec![2, 1],
            vec![4, 2],
            vec![6, 3],
        ];
        let result = Solution::check_straight_line(coordinates);
        assert!(result);
    }

    #[test]
    fn test_input_6() {
        #[rustfmt::skip]
        let coordinates = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 5],
        ];
        let result = Solution::check_straight_line(coordinates);
        assert!(!result);
    }

    #[test]
    fn test_input_48() {
        #[rustfmt::skip]
        let coordinates = vec![
            vec![2, 4],
            vec![2, 5],
            vec![2, 8],
        ];
        let result = Solution::check_straight_line(coordinates);
        assert!(result);
    }
}
