use std::collections::BTreeMap;

#[allow(dead_code)]
impl Solution {
    // 1. sort vec with indexes and collect them
    // 2. sort heights by key, get default indexes and pick the names
    #[rustfmt::skip]
    pub fn sort_people(
        names: Vec<String>,
        heights: Vec<i32>
    ) -> Vec<String> {
        let mut map = BTreeMap::new();

        for (i, n) in names.into_iter().enumerate() {
            map.insert(-heights[i], n);
        }
        map.into_values().collect()

        /*
        let mut with_index = names
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, String)>>();

        with_index.sort_by(|(a, _), (b, _)| {
            heights[*b].cmp(&heights[*a])
        });

        with_index
            .into_iter()
            .map(|(_, s)| s)
            .collect::<Vec<String>>()
        */
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
        let names = vec![
            "Mary".to_string(),
            "John".to_string(),
            "Emma".to_string(),
        ];
        let heights = vec![180, 165, 170];

        let result = Solution::sort_people(names, heights);
        #[rustfmt::skip]
        assert_eq!(
            vec![
                "Mary".to_string(),
                "Emma".to_string(),
                "John".to_string(),
            ],
            result
        );
    }

    #[test]
    fn test_example_2() {
        #[rustfmt::skip]
        let names = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Bob".to_string(),
        ];
        let heights = vec![155, 185, 150];

        let result = Solution::sort_people(names, heights);

        #[rustfmt::skip]
        assert_eq!(
            vec![
                "Bob".to_string(),
                "Alice".to_string(),
                "Bob".to_string(),
            ],
            result
        );
    }
}
