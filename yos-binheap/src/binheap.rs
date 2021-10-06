use super::hole::Hole;
use super::iter::*;

use core::iter::FromIterator;
use core::mem::swap;
use std::fmt::Debug;

//
// BinHeap
//

/// Implementation of the binary heap.
pub struct BinHeap<T> {
    data: Vec<T>,
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

// VLD Revert the debug
impl<T: Debug + Ord> Default for BinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Debug for BinHeap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

// VLD Revert the debug
impl<T: Debug + Ord> BinHeap<T> {
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
            let mut hole = Hole::new(&mut self.data, pos);

            while hole.pos() > start {
                let parent = (hole.pos() - 1) / 2;

                if hole.element() <= hole.get(parent) {
                    break;
                }

                hole.move_to(parent);
            }

            hole.pos()
        }
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    fn sift_down_range(&mut self, pos: usize, end: usize) {
        unsafe {
            let mut hole = Hole::new(&mut self.data, pos);
            let mut child = 2 * pos + 1;
            while child < end {
                let right = child + 1;
                // compare with the greater of the two children
                if right < end && (hole.get(child) <= hole.get(right)) {
                    child = right;
                }
                // if we are already in order, stop.
                if hole.element() >= hole.get(child) {
                    break;
                }
                hole.move_to(child);
                child = 2 * hole.pos() + 1;
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
            let mut hole = Hole::new(&mut self.data, pos);
            let mut child = 2 * pos + 1;
            while child < end {
                let right = child + 1;
                // compare with the greater of the two children
                if right < end && (hole.get(child) <= hole.get(right)) {
                    child = right;
                }
                hole.move_to(child);
                child = 2 * hole.pos() + 1;
            }

            pos = hole.pos();
        }
        self.sift_up(start, pos);
    }

    fn rebuild(&mut self) {
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

//
// Converters
//
// VLD Revert the debug
impl<T: Debug + Ord> From<Vec<T>> for BinHeap<T> {
    /// Converts a `Vec<T>` into a `BinaryHeap<T>`.
    ///
    /// This conversion happens in-place, and has `O(n)` time complexity.
    fn from(vec: Vec<T>) -> Self {
        let mut heap = Self { data: vec };
        heap.rebuild();
        heap
    }
}

impl<T> From<BinHeap<T>> for Vec<T> {
    fn from(heap: BinHeap<T>) -> Vec<T> {
        heap.data
    }
}

// VLD Revert te debug
impl<T: Debug + Ord> FromIterator<T> for BinHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> BinHeap<T> {
        BinHeap::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<T> IntoIterator for BinHeap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter::new(self.data.into_iter())
    }
}

impl<'a, T> IntoIterator for &'a BinHeap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}
