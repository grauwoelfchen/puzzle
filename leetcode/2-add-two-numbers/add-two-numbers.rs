// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    const RADIX: u32 = 10;

    fn list_to_number(l: Box<ListNode>, acc: Option<String>) -> String {
        let n = if let Some(a) = acc {
            format!("{}{}", l.val, a)
        } else {
            l.val.to_string()
        };
        if l.next.is_none() {
            return n;
        }
        Self::list_to_number(l.next.unwrap(), Some(n))
    }

    fn number_to_list(n: String, v: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if n.is_empty() {
            return v;
        }
        let chars: Vec<i32> = n
            .chars()
            .map({ |c| c.to_digit(Self::RADIX).unwrap() as i32 })
            .collect();

        let nn: String = chars[1..chars.len()]
            .iter()
            .map({ |i| i.to_string() })
            .collect();
        let nv = Some(Box::new(ListNode {
            next: v,
            val: chars[0] as i32,
        }));
        Self::number_to_list(nn, nv)
    }

    // 340282366920938463463374607431768211455
    // 2432432432432432432432432432432432432432432432432432432432439
    fn add_numbers(n1: String, n2: String) -> String {
        let mut result = "".to_string();
        let max = n1.len().max(n2.len());

        let mut num1: Vec<char> = n1.chars().collect();
        let mut num2: Vec<char> = n2.chars().collect();

        let mut overflow = 0;
        for _ in 0..max {
            let left: u8 = match num1.pop() {
                Some(c) => c.to_string().parse().unwrap(),
                None => 0,
            };
            let right: u8 = match num2.pop() {
                Some(c) => c.to_string().parse().unwrap(),
                None => 0,
            };
            let mut sum: String = (left + right + overflow)
                .to_string()
                .chars()
                .rev()
                .collect();

            overflow = if sum.len() > 1 {
                sum.pop().unwrap().to_string().parse::<u8>().unwrap()
            } else {
                0
            };
            result.push(*sum.chars().collect::<Vec<char>>().first().unwrap());
        }
        if overflow > 0 {
            result.push(
                *overflow
                    .to_string()
                    .chars()
                    .collect::<Vec<char>>()
                    .first()
                    .unwrap(),
            );
        }
        result.chars().rev().collect::<String>()
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(l1v) = l1 {
            if let Some(l2v) = l2 {
                let n1 = Self::list_to_number(l1v, None);
                let n2 = Self::list_to_number(l2v, None);
                // let n = n1.parse::<u128>().unwrap() + n2.parse::<u128>().unwrap();
                let n = Self::add_numbers(n1, n2);
                return Self::number_to_list(n, None);
            }
        }
        None
    }
}
