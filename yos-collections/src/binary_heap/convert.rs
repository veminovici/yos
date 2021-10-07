use super::BinaryHeap;
use core::iter::FromIterator;

impl<T: Ord> From<Vec<T>> for BinaryHeap<T> {
    /// Converts a `Vec<T>` into a `BinaryHeap<T>`.
    ///
    /// This conversion happens in-place, and has `O(n)` time complexity.
    fn from(vec: Vec<T>) -> Self {
        let mut heap = Self { data: vec };
        heap.rebuild();
        heap
    }
}

impl<T> From<BinaryHeap<T>> for Vec<T> {
    fn from(heap: BinaryHeap<T>) -> Vec<T> {
        heap.data
    }
}

impl<T: Ord> FromIterator<T> for BinaryHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> BinaryHeap<T> {
        BinaryHeap::from(iter.into_iter().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_heap::BinaryHeap;

    #[test]
    fn from_vec_to_vec() {
        let mut h = BinaryHeap::from(vec![3, 1, 4]);
        h.push(2);

        let xs: Vec<i32> = h.into();
        assert_eq!(xs.len(), 4);
    }

    #[test]
    fn from_iterator() {
        let xs = vec![3, 1, 4];
        let mut h = BinaryHeap::from_iter(xs.into_iter());

        h.push(2);

        let xs: Vec<i32> = h.into();
        assert_eq!(xs.len(), 4);
    }
}
