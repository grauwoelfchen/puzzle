use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
impl Solution {
    #[rustfmt::skip]
    fn find(tree: &Option<Rc<RefCell<TreeNode>>>, j: i32) -> bool {
        match tree {
            Some(subtree) => {
                let subtree = subtree.borrow();
                let v = subtree.val;
                if v == j {
                    return true;
                }
                (j < v && Self::find(&subtree.left, j)) ||
                (j > v && Self::find(&subtree.right, j))
            }
            _ => false,
        }
    }

    #[rustfmt::skip]
    fn find_n(
        tree: &Option<Rc<RefCell<TreeNode>>>,
        root: &Option<Rc<RefCell<TreeNode>>>,
        n: i32,
    ) -> bool {
        match tree.clone() {
            Some(subtree) => {
                let subtree = subtree.borrow();

                let v = subtree.val;
                let j = n - v;
                if j == n {
                    return false;
                }
                if Self::find(&root, j) {
                    return true;
                }
                Self::find_n(&subtree.left, &root, n) ||
                Self::find_n(&subtree.right, &root, n)
            },
            None => false,
        }
    }

    #[rustfmt::skip]
    pub fn find_target(
        root: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> bool {
        match root.clone() {
            Some(tree) => {
                let tree = tree.borrow();

                let v = tree.val;
                let j = k - v;
                if (j < v && Self::find(&tree.left, j)) ||
                   (j > v && Self::find(&tree.right, j)) {
                    return true;
                }
                Self::find_n(&tree.left, &root, k) ||
                Self::find_n(&tree.right, &root, k)
            },
            None => false,
        }
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
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let k = 9;
        let result = Solution::find_target(root, k);
        assert!(result);
    }

    #[test]
    fn test_example_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let k = 28;
        let result = Solution::find_target(root, k);
        assert!(!result);
    }

    #[test]
    fn test_example_299() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let k = 4;
        let result = Solution::find_target(root, k);
        assert!(result);
    }
}
