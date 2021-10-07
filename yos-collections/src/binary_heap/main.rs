//! Implementation of the binary heap
use super::iter::*;
use yos_memcursor::MemCursor;

use core::mem::swap;
use std::fmt::Debug;

/// Implementation of the binary heap.
pub struct BinaryHeap<T> {
    pub(crate) data: Vec<T>,
}

impl<T: Clone> Clone for BinaryHeap<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
    }
}

impl<T: Ord> Default for BinaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Debug for BinaryHeap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Ord> BinaryHeap<T> {
    /// Creates a new new instance of the binary heap.
    ///
    /// # Example
    ///
    /// ```
    /// use yos_collections::binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::new();
    /// heap.push(1);
    /// heap.push(3);
    /// heap.push(2);
    ///
    /// assert_eq!(3, heap.len());
    /// ```
    pub fn new() -> Self {
        Self {
            data: Vec::<T>::new(),
        }
    }

    /// Creates a new instance of the binary heap with a given capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use yos_collections::binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::with_capacity(3);
    /// heap.push(1);
    /// heap.push(3);
    /// heap.push(2);
    ///
    /// assert_eq!(3, heap.len());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::<T>::with_capacity(capacity),
        }
    }

    /// Removes the greatest item from the binary heap and returns it,
    /// or `None` if it is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use yos_collections::binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::with_capacity(3);
    ///
    /// let v = heap.pop();
    /// assert_eq!(None, v);
    ///
    /// heap.push(1);
    /// heap.push(3);
    /// heap.push(2);
    ///
    /// let v = heap.pop().unwrap();
    /// assert_eq!(3, v);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        // Get the last element, a small one.
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                // Swap the small element with the head of the heap.
                swap(&mut item, &mut self.data[0]);

                // Now the item will have the largest value and the
                // small value will be at the head of the heap.
                // Bubble down the head to fix the heap.
                self.bubble_down(0);
            }

            item
        })
    }

    /// Pushes a value into the binary heap.
    ///
    /// # Example
    ///
    /// ```
    /// use yos_collections::binary_heap::BinaryHeap;
    ///
    /// let mut heap = BinaryHeap::with_capacity(3);
    /// heap.push(1);
    /// heap.push(3);
    /// heap.push(2);
    ///
    /// let v = heap.pop().unwrap();
    /// assert_eq!(3, v);
    /// ```
    pub fn push(&mut self, value: T) {
        let old_len = self.len();
        self.data.push(value);

        self.bubble_up(0, old_len);
    }

    /// Bubbles up into the heap the value at 'pos' up to the 'start' index.
    fn bubble_up(&mut self, start: usize, pos: usize) {
        unsafe {
            // take out the value at 'pos' and create a hole.
            let mut mc = MemCursor::new(&mut self.data, pos);

            while mc.pos() > start {
                let parent = (mc.pos() - 1) / 2;

                if mc.element() <= mc.get(parent) {
                    break;
                }

                mc.move_to(parent);
            }
        }
    }

    /// Bubbles down into the heap the value at 'pos'
    fn bubble_down(&mut self, pos: usize) {
        let end = self.len();

        unsafe {
            let mut mc = MemCursor::new(&mut self.data, pos);
            let mut child = 2 * pos + 1;
            while child < end {
                let child1 = child + 1;
                // compare with the greater of the two children
                if child1 < end && (mc.get(child) <= mc.get(child1)) {
                    child = child1;
                }

                if mc.get(child) < mc.element() {
                    break;
                }

                mc.move_to(child);
                child = 2 * mc.pos() + 1;
            }
        }
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    fn sift_down_range(&mut self, pos: usize, end: usize) {
        unsafe {
            let mut mc = MemCursor::new(&mut self.data, pos);

            let mut child = 2 * pos + 1;
            while child < end {
                let right = child + 1;
                // compare with the greater of the two children
                if right < end && (mc.get(child) <= mc.get(right)) {
                    child = right;
                }

                // if we are already in order, stop.
                if mc.get(child) < mc.element() {
                    break;
                }
                mc.move_to(child);
                child = 2 * mc.pos() + 1;
            }
        }
    }

    /// Rebuilds the whole heap, from scratch.
    pub(crate) fn rebuild(&mut self) {
        let mut n = self.len() / 2;
        while n > 0 {
            n -= 1;
            //self.sift_down(n);
            self.sift_down_range(n, self.len());
        }
    }
}

impl<T> BinaryHeap<T> {
    /// Returns an iterator that is visiting all values in the binary heap.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.data.iter())
    }

    /// Retruns the greatest element in the binary heap.
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Returns the number of elements the binary heap can hold.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Reserves the minimum capacity for additional more elements
    /// to be inserted in the binary heap.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }

    /// Reserver additional space.
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Shrinks the allocated memory.
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Returns the number of elements in the binary heap.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns a flag indicating whenever the binary heap is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_pass() {
        let h = BinaryHeap::<u8>::new();
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());
    }

    #[test]
    fn with_capacity_pass() {
        let h = BinaryHeap::<u8>::with_capacity(10);
        assert_eq!(h.capacity(), 10);
    }

    #[test]
    fn default_pass() {
        let h = BinaryHeap::<u8>::default();
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());
    }

    #[test]
    fn debug_pass() {
        let h = BinaryHeap::<u8>::new();
        let s = format!("{:?}", h);
        assert!(!s.is_empty());
    }

    #[test]
    fn push_pass() {
        let mut h = BinaryHeap::<u8>::new();
        h.push(4);
        h.push(2);
        h.push(3);
        h.push(5);

        assert_eq!(h.len(), 4);
        assert!(!h.is_empty());

        let v = h.peek().unwrap();
        assert_eq!(*v, 5);
    }

    #[test]
    fn pop_pass() {
        let mut h = BinaryHeap::<u8>::new();
        h.push(4);
        h.push(2);
        h.push(3);
        h.push(5);

        let v = h.pop().unwrap();
        assert_eq!(5, v);

        let v = h.pop().unwrap();
        assert_eq!(4, v);

        let v = h.pop().unwrap();
        assert_eq!(3, v);

        let v = h.pop().unwrap();
        assert_eq!(2, v);

        let v = h.pop();
        assert_eq!(None, v);
    }

    #[test]
    fn rebuild_pass() {
        let mut h = BinaryHeap::from(vec![1, 3, 5, 2, 4]);
        assert_eq!(5, h.len());

        let v = h.pop().unwrap();
        assert_eq!(5, v);

        let v = h.pop().unwrap();
        assert_eq!(4, v);

        let v = h.pop().unwrap();
        assert_eq!(3, v);

        let v = h.pop().unwrap();
        assert_eq!(2, v);

        let v = h.pop().unwrap();
        assert_eq!(1, v);

        let v = h.pop();
        assert_eq!(None, v);
    }
}
