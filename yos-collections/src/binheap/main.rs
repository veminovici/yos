//! Implementation of the binary heap
use super::iter::*;
use yos_memcursor::MemCursor;

use core::mem::swap;
use std::fmt::Debug;

/// Implementation of the binary heap.
pub struct BinHeap<T> {
    pub(crate) data: Vec<T>,
}

impl<T: Clone> Clone for BinHeap<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
    }
}

impl<T: Ord> Default for BinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Debug for BinHeap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Ord> BinHeap<T> {
    /// Creates a new new instance of the binary heap.
    pub fn new() -> Self {
        Self {
            data: Vec::<T>::new(),
        }
    }

    /// Creates a new instance of the binary heap with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::<T>::with_capacity(capacity),
        }
    }

    /// Removes the greatest item from the binary heap and returns it,
    /// or `None` if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                swap(&mut item, &mut self.data[0]);
                self.sift_down_to_bottom(0);
            }
            item
        })
    }

    /// Pushes a value into the binary heap.
    pub fn push(&mut self, value: T) {
        let old_len = self.len();
        self.data.push(value);
        self.sift_up(0, old_len);
    }

    fn sift_up(&mut self, start: usize, pos: usize) -> usize {
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

            mc.pos()
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
                if mc.element() >= mc.get(child) {
                    break;
                }
                mc.move_to(child);
                child = 2 * mc.pos() + 1;
            }
        }
    }

    fn sift_down(&mut self, pos: usize) {
        let len = self.len();
        self.sift_down_range(pos, len);
    }

    fn sift_down_to_bottom(&mut self, mut pos: usize) {
        let end = self.len();
        let start = pos;
        unsafe {
            let mut mc = MemCursor::new(&mut self.data, pos);
            let mut child = 2 * pos + 1;
            while child < end {
                let right = child + 1;
                // compare with the greater of the two children
                if right < end && (mc.get(child) <= mc.get(right)) {
                    child = right;
                }
                mc.move_to(child);
                child = 2 * mc.pos() + 1;
            }

            pos = mc.pos();
        }
        self.sift_up(start, pos);
    }

    pub(crate) fn rebuild(&mut self) {
        let mut n = self.len() / 2;
        while n > 0 {
            n -= 1;
            self.sift_down(n);
        }
    }
}

impl<T> BinHeap<T> {
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
