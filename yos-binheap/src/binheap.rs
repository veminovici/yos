use crate::ndx::{bheap_is_root, bheap_parent_ndx};
use std::fmt::Debug;

/// Implementation of the binary heap.
pub struct BinHeap<T> {
    data: Vec<T>,
}

impl<T: Debug> Debug for BinHeap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bheap: {:?}", self.data)
    }
}

impl<T: Copy + PartialOrd> Default for BinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + PartialOrd> BinHeap<T> {
    /// Create a new instance of the binary heap.
    ///
    /// # Example
    ///
    ///```
    /// use yos_binheap::BinHeap;
    ///
    /// let heap = BinHeap::<u8>::new();
    /// assert_eq!(heap.len(), 0);
    /// ```
    pub fn new() -> Self {
        BinHeap { data: vec![] }
    }

    /// Returns the size of the binary heap.
    #[inline]
    ///
    /// # Example
    ///
    ///```
    /// use yos_binheap::BinHeap;
    ///
    /// let mut heap = BinHeap::<u8>::new();
    /// heap.push(1);
    ///
    /// assert_eq!(heap.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the binary heap is empty
    ///
    /// # Example
    ///
    ///```
    /// use yos_binheap::BinHeap;
    ///
    /// let mut heap = BinHeap::<u8>::new();
    /// assert!(heap.is_empty());
    ///
    /// heap.push(1);
    /// assert!(!heap.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Pushes a new item in the binary heap.
    ///
    /// # Example
    ///
    ///```
    /// use yos_binheap::BinHeap;
    ///
    /// let mut heap = BinHeap::<u8>::new();
    /// assert!(heap.is_empty());
    ///
    /// heap.push(1);
    /// assert!(!heap.is_empty());
    /// ```
    pub fn push(&mut self, item: T) {
        // add the item to the bottom of the heap vector.
        self.data.push(item);

        // bubble up the item in the heap.
        self.bubble_up(self.data.len() - 1);
    }

    /// Bubbles up in the binary heap the element at a given position.
    fn bubble_up(&mut self, pos: usize) {
        if bheap_is_root(pos) {
            return;
        }

        let parent = bheap_parent_ndx(pos);
        if self.data[parent] >= self.data[pos] {
            return;
        }

        let mut a = self.data[pos];
        let mut b = self.data[parent];

        std::mem::swap(&mut a, &mut b);

        self.data[pos] = a;
        self.data[parent] = b;
    }
}
