/*
    heap
    This question requires you to implement a binary heap function
*/

use std::clone::Clone;
use std::cmp::Ord;
use std::default::Default;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Heap<T>
where
    T: Default + Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Ord + std::clone::Clone + Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        let mut i = self.count;

        while i > 1 {
            let p = self.parent_idx(i);
            if (self.comparator)(&self.items[p], &self.items[i]) {
                break;
            }
            self.items.swap(p, i);
            i = p;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let mut smallest_idx = idx;

        let left_child_idx = self.left_child_idx(idx);

        if left_child_idx <= self.count
            && (self.comparator)(&self.items[left_child_idx], &self.items[smallest_idx])
        {
            smallest_idx = left_child_idx;
        }

        let right_child_idx = self.right_child_idx(idx);

        if right_child_idx <= self.count
            && (self.comparator)(&self.items[right_child_idx], &self.items[smallest_idx])
        {
            smallest_idx = right_child_idx;
        }

        smallest_idx
    }

    fn fix_down(&mut self, idx: usize) {
        let mut i = idx;
        loop {
            let s = self.smallest_child_idx(i);
            if (self.comparator)(&self.items[s], &self.items[i]) {
                self.items.swap(s, i);
                i = s;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + std::clone::Clone + Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

// the iterator should not modify the `Heap`,
// the correct way is to create a `Iter` structure to handle iteration, `Heap` implements a `iter`
// method to return the `Iter`. And we could implement `IntoIter` trait.
impl<T> Iterator for Heap<T>
where
    T: Default + Ord + Clone + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count > 0 {
            let v = self.items.swap_remove(1);
            self.count -= 1;
            self.fix_down(1);
            return Some(v);
        }
        None
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone + Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone + Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
