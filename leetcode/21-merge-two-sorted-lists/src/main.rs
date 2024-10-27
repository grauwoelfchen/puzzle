#[allow(dead_code)]
impl Solution {
    fn do_something(list: &mut Option<Box<ListNode>>) {
        // println!(">>> {:#?}", (*node).next);
        let node = list.as_mut().unwrap();
        node.next = Some(Box::new(ListNode::new(999)));
    }

    #[allow(unused_variables)]
    fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // let list: &mut Option<Box<ListNode>> = &mut list1.clone();
        // Self::do_something(list);
        // dbg!(&list);

        // solution
        match (list1, list2) {
            (None, None) => None,
            (Some(l1), None) => Some(l1), // sorted
            (None, Some(l2)) => Some(l2), // sorted
            (Some(l1), Some(l2)) => {
                if l1.val < l2.val {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Self::merge_two_lists(l1.next, Some(l2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Self::merge_two_lists(Some(l1), l2.next),
                    }))
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
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
        let list1 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode::new(4))),
                val: 2,
            })),
            val: 1,
        }));
        let list2 = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode::new(4))),
                val: 3,
            })),
            val: 1,
        }));
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(
            Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode {
                        next: Some(Box::new(ListNode {
                            next: Some(Box::new(ListNode {
                                next: Some(Box::new(ListNode::new(4))),
                                val: 4,
                            })),
                            val: 3,
                        })),
                        val: 2,
                    })),
                    val: 1
                })),
                val: 1,
            })),
            result
        );
    }

    #[test]
    fn test_example_2() {
        let list1 = None;
        let list2 = None;
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(None, result);
    }

    #[test]
    fn test_example_3() {
        let list1 = None;
        let list2 = Some(Box::new(ListNode::new(0)));
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(Some(Box::new(ListNode::new(0))), result);
    }
}
