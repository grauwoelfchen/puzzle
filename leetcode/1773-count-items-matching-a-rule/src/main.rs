#[allow(dead_code)]
impl Solution {
    // type color name
    #[rustfmt::skip]
    pub fn count_matches(
			items: Vec<Vec<String>>,
			rule_key: String,
			rule_value: String,
		) -> i32 {
			items.into_iter().filter(|x| {
			    match rule_key.as_str() {
			    	  "type" => x[0] == rule_value,
			    	  "color" => x[1] == rule_value,
              "name" => x[2] == rule_value,
			    	  _ => false
			    }
			}).count() as i32
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
        let items: Vec<Vec<String>> = vec![
            vec![
                "phone".to_string(),
                "blue".to_string(),
                "pixel".to_string()],
            vec![
                "computer".to_string(),
                "silver".to_string(),
                "lenovo".to_string(),
            ],
            vec![
                "phone".to_string(),
                "gold".to_string(),
                "iphone".to_string(),
            ],
        ];
        let rule_key = "color".to_string();
        let rule_value = "silver".to_string();
        let result = Solution::count_matches(items, rule_key, rule_value);
        assert_eq!(1, result);
    }

    #[test]
    fn test_example_2() {
        #[rustfmt::skip]
        let items: Vec<Vec<String>> = vec![
            vec![
                "phone".to_string(),
                "blue".to_string(),
                "pixel".to_string()
            ],
            vec![
                "computer".to_string(),
                "silver".to_string(),
                "phone".to_string(),
            ],
            vec![
                "phone".to_string(),
                "gold".to_string(),
                "iphone".to_string(),
            ],
        ];
        let rule_key = "type".to_string();
        let rule_value = "phone".to_string();
        let result = Solution::count_matches(items, rule_key, rule_value);
        assert_eq!(2, result);
    }
}
