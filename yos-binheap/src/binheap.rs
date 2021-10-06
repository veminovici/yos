use core::iter::FromIterator;
use core::ptr;
use std::fmt::Debug;
use std::mem::ManuallyDrop;
use std::slice;

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
                if right < end && !(hole.get(child) > hole.get(right)) {
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
        Iter {
            iter: self.data.iter(),
        }
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

    /// Shrinks to a minimum capacity.
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity);
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
// Hole
//

/// Hole represents a hole in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<'a, T: 'a> {
    data: &'a mut [T],
    elt: ManuallyDrop<T>,
    pos: usize,
}

impl<'a, T> Hole<'a, T> {
    #[inline]
    unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
        debug_assert!(pos < data.len());

        // SAFE: pos should be inside the slice
        let elt = ptr::read(data.get_unchecked(pos));
        Hole {
            data,
            elt: ManuallyDrop::new(elt),
            pos,
        }
    }

    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    fn element(&self) -> &T {
        &self.elt
    }

    /// Returns a reference to the element at `index`.
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn get(&self, index: usize) -> &T {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());

        self.data.get_unchecked(index)
    }

    /// Move hole to new location
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn move_to(&mut self, index: usize) {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());

        let index_ptr: *const _ = self.data.get_unchecked(index);
        let hole_ptr = self.data.get_unchecked_mut(self.pos);
        ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
        self.pos = index;
    }
}

//
// Iterators
//

/// An iterator over the elements in a binary heap.
pub struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>,
}

impl<T: Debug> Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Iter").field(&self.iter.as_slice()).finish()
    }
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<&'a T> {
        self.iter.last()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back()
    }
}

/// An owning iterator over the elements of a `BinaryHeap`.
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: std::vec::IntoIter<T>,
}

impl<T: Debug> Debug for IntoIter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IntoIter")
            .field(&self.iter.as_slice())
            .finish()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

//
// Converters
//

impl<T: Ord> From<Vec<T>> for BinHeap<T> {
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

impl<T: Ord> FromIterator<T> for BinHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> BinHeap<T> {
        BinHeap::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<T> IntoIterator for BinHeap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            iter: self.data.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a BinHeap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}
