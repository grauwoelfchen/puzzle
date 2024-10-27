#[allow(dead_code)]
struct MyHashMap {
    // data: Vec<i32>,
    data: Vec<(i32, i32)>,
}

#[allow(dead_code)]
impl MyHashMap {
    fn new() -> Self {
        Self { data: vec![] }
    }

    // (key, (key, value))
    fn search(&self, key: i32) -> (Option<usize>, (i32, i32)) {
        if self.data.is_empty() {
            return (None, (-1, -1));
        }
        let mut low: i32 = 0;
        let mut high: i32 = self.data.len() as i32 - 1;
        while low <= high {
            let mid = (low + high) / 2;

            let (k, v) = self.data[mid as usize];
            if k == key {
                return (Some(mid as usize), (k, v));
            }
            if k > key {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        (None, (-1, -1))
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.search(key) {
            (Some(k), _) => {
                self.data[k] = (key, value);
            }
            _ => {
                self.data.push((key, value));
            }
        }
        self.data.sort_by(|(a, _), (b, _)| a.cmp(b));
    }

    fn get(&self, key: i32) -> i32 {
        match self.search(key) {
            (Some(_), (_, v)) => v,
            _ => -1,
        }
    }

    fn remove(&mut self, key: i32) {
        match self.search(key) {
            (Some(k), _) => {
                let _ = self.data.swap_remove(k);
                self.data.sort_by(|(a, _), (b, _)| a.cmp(b));
            }
            _ => (),
        }
    }
}

/*
#[allow(dead_code)]
impl MyHashMap {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn hash(&self, key: i32) -> usize {
        key as usize
    }

    fn put(&mut self, key: i32, value: i32) {
        let k = self.hash(key);
        match self.data.get(k) {
            Some(_) => self.data[k] = value,
            None if k >= self.data.len() => {
                self.data.resize(k + 1, -1);
                self.data.insert(k, value);
            }
            _ => self.data.insert(k, value),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let k = self.hash(key);
        match self.data.get(k) {
            Some(v) => *v,
            _ => -1,
        }
    }

    fn remove(&mut self, key: i32) {
        if key >= self.data.len() as i32 {
            return;
        }

        self.put(key, -1);

        //let k = self.hash(key);

        // O(n) in a worst-case
        // let _ = self.data.remove(k);

        // replace the slot with the last element (not ordered)
        // let _ = self.data.swap_remove(k);
    }
}
*/

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

// struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_put() {
        let mut obj = MyHashMap::new();
        obj.put(1, 1);
        assert_eq!(obj.get(1), 1);

        obj.remove(1);
        assert_eq!(obj.get(1), -1);
    }
}
