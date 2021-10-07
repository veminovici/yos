use super::{BinHeap, IntoIter, Iter};
use core::iter::FromIterator;

// $$VLD - Remove debug
impl<T: std::fmt::Debug + Ord> From<Vec<T>> for BinHeap<T> {
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

// $$VLD - Remove debug
impl<T: std::fmt::Debug + Ord> FromIterator<T> for BinHeap<T> {
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
