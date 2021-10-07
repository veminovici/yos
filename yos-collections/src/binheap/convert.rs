use super::BinHeap;
use core::iter::FromIterator;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binheap::BinHeap;

    #[test]
    fn from_vec_to_vec() {
        let mut h = BinHeap::from(vec![3, 1, 4]);
        h.push(2);

        let xs: Vec<i32> = h.into();
        assert_eq!(xs.len(), 4);
    }

    #[test]
    fn from_iterator() {
        let xs = vec![3, 1, 4];
        let mut h = BinHeap::from_iter(xs.into_iter());

        h.push(2);

        let xs: Vec<i32> = h.into();
        assert_eq!(xs.len(), 4);
    }
}
