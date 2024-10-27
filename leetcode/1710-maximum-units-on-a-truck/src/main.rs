#[allow(dead_code)]
impl Solution {
    // 4
    //
    // traverse all results
    //
    // [[1, 3], [2, 2], [3, 1]]
    //   -> 8
    // 
    // -> [3, 4, 3] total 10 (box 6)
    // -> [0, 4, 3] total  6 (box 3)
    // -> [3, 2, 3] total  8 (box 5)
    // -> [3, 0, 3] total  6 (box 4)
    // -> [3, 4, 2] total  9 (box 5)
    // -> [3, 4, 1] total  8 (box 4)
    // -> [3, 4, 0] total  7 (box 3)
    //
    // --
    // sort
    // pop
    //
    // [[2, 3], [2, 2], [3, 1]]
    //   -> [[3, 1], [2, 2], [2, 3]]
    //   -> 8
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut units = 0;
        if box_types.is_empty() {
            return units;
        }

        let data = &mut box_types.clone();
        data.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut nob = 0;
        // x fold (acc)
        while let Some(elm) = data.pop() {
            // FIXME
            for _ in 0..elm[0] {
                nob += 1;
                if nob > truck_size {
                    return units;
                }
                units += elm[1];
            }
        }
        units
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
        let box_types = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
        let truck_size = 4;
        let result = Solution::maximum_units(box_types, truck_size);
        assert_eq!(8, result);
    }

    #[test]
    fn test_example_2() {
        let box_types = vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]];
        let truck_size = 10;
        let result = Solution::maximum_units(box_types, truck_size);
        assert_eq!(91, result);
    }
}
