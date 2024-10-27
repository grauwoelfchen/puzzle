use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    fn is_connected(
        image: &Vec<Vec<i32>>,
        sr: usize,
        sc: usize,
        r: usize,
        c: usize,
        t: i32,
        checked: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if let Some(row) = image.get(r) {
            if row[c] != t {
                return false;
            }

            // neighbors
            let up = if r > 0 {
                let n = r - 1;
                if (n, c) == (sr, sc) {
                    return true;
                }
                if checked.contains(&(n, c)) {
                    false
                } else {
                    checked.insert((n, c));

                    match image[n].get(c) {
                        Some(v) if *v == t => {
                            Self::is_connected(&image, sr, sc, n, c, t, checked)
                        }
                        _ => false,
                    }
                }
            } else {
                false
            };

            let right = if c < (row.len() - 1) {
                let n = c + 1;
                if (r, n) == (sr, sc) {
                    return true;
                }
                if checked.contains(&(r, n)) {
                    false
                } else {
                    checked.insert((r, n));

                    match row.get(n) {
                        Some(v) if *v == t => {
                            Self::is_connected(&image, sr, sc, r, n, t, checked)
                        }
                        _ => false,
                    }
                }
            } else {
                false
            };

            let down = if r < (image.len() - 1) {
                let n = r + 1;
                if (n, c) == (sr, sc) {
                    return true;
                }
                if checked.contains(&(n, c)) {
                    false
                } else {
                    checked.insert((n, c));

                    match image[n].get(c) {
                        Some(v) if *v == t => {
                            Self::is_connected(&image, sr, sc, n, c, t, checked)
                        }
                        _ => false,
                    }
                }
            } else {
                false
            };

            let left = if c > 0 {
                let n = c - 1;
                if (r, n) == (sr, sc) {
                    return true;
                }
                if checked.contains(&(r, n)) {
                    false
                } else {
                    checked.insert((r, n));

                    match row.get(n) {
                        Some(v) if *v == t => {
                            Self::is_connected(&image, sr, sc, r, n, t, checked)
                        }
                        _ => false,
                    }
                }
            } else {
                false
            };

            return up || right || down || left;
        }
        false
    }

    pub fn flood_fill(
        image: Vec<Vec<i32>>,
        sr: i32,
        sc: i32,
        color: i32,
    ) -> Vec<Vec<i32>> {
        let sr = sr as usize;
        let sc = sc as usize;

        let t = image[sr][sc];
        if t == color {
            return image;
        }

        let mut image = image;
        let mut turned: HashSet<(usize, usize)> = HashSet::new();
        turned.insert((sr, sc));

        for r in 0..(image.len()) {
            for c in 0..(image[r].len()) {
                if r == sr && c == sc {
                    continue;
                }
                let mut checked: HashSet<(usize, usize)> = HashSet::new();
                if Self::is_connected(&image, sr, sc, r, c, t, &mut checked) {
                    turned.insert((r, c));
                }
            }
        }
        for (r, c) in turned {
            image[r][c] = color;
        }
        image
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
        #[rustfmt::skip]
        let image = vec![
            vec![1, 1, 1],
            vec![1, 1, 0],
            vec![1, 0, 1],
        ];
        let sr = 1;
        let sc = 1;
        let color = 2;
        let result = Solution::flood_fill(image, sr, sc, color);

        #[rustfmt::skip]
        assert_eq!(vec![
            vec![2, 2, 2],
            vec![2, 2, 0],
            vec![2, 0, 1]
        ], result);
    }

    #[test]
    fn test_example_2() {
        #[rustfmt::skip]
        let image = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        let sr = 0;
        let sc = 0;
        let color = 0;
        let result = Solution::flood_fill(image, sr, sc, color);

        #[rustfmt::skip]
        assert_eq!(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
        ], result);
    }

    #[test]
    fn test_example_141() {
        #[rustfmt::skip]
        let image = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        let sr = 1;
        let sc = 0;
        let color = 2;
        let result = Solution::flood_fill(image, sr, sc, color);

        #[rustfmt::skip]
        assert_eq!(vec![
            vec![2, 2, 2],
            vec![2, 2, 2],
        ], result);
    }

    #[test]
    fn test_example_235() {
        #[rustfmt::skip]
        let image = vec![
            vec![0, 0, 1],
            vec![1, 0, 1],
        ];
        let sr = 1;
        let sc = 0;
        let color = 2;
        let result = Solution::flood_fill(image, sr, sc, color);

        #[rustfmt::skip]
        assert_eq!(vec![
            vec![0, 0, 1],
            vec![2, 0, 1],
        ], result);
    }

    #[test]
    fn test_example_260() {
        #[rustfmt::skip]
        let image = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
        ];
        let sr = 1;
        let sc = 1;
        let color = 1;
        let result = Solution::flood_fill(image, sr, sc, color);

        #[rustfmt::skip]
        assert_eq!(vec![
            vec![1, 1, 0],
            vec![1, 1, 1],
        ], result);
    }
}
