struct KthLargest {
    k: usize,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let k = k as usize;

        nums.sort_by(|a, b| b.cmp(&a));

        if let Some(_) = nums.get(k - 1) {
            nums = Vec::from(&nums[0..k]);
        }

        Self {
            k: k as usize,
            nums,
        }
    }

    #[rustfmt::skip]
    fn add(&mut self, val: i32) -> i32 {
        match self.nums.get(self.k - 1) {
            Some(n) if n >= &val => {
                *n
            }
            _ =>  {
                self.nums.push(val);
                self.nums.sort_by(|a, b| b.cmp(&a));
                self.nums = Vec::from(&self.nums[0..self.k]);
                self.nums[self.k - 1]
            }
        }
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

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
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(4, kth.add(3));
        assert_eq!(5, kth.add(5));
        assert_eq!(5, kth.add(10));
        assert_eq!(8, kth.add(9));
        assert_eq!(8, kth.add(4));
    }

    #[test]
    fn test_example_2() {
        let mut kth = KthLargest::new(1, vec![]);
        assert_eq!(-3, kth.add(-3));
        assert_eq!(-2, kth.add(-2));
        assert_eq!(-2, kth.add(-4));
        assert_eq!(0, kth.add(0));
        assert_eq!(4, kth.add(4));
    }
}
