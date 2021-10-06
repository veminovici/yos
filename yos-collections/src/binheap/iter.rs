/// An iterator over the elements in a binary heap.
use std::fmt::Debug;
use std::slice;

/// Iterator
pub struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>,
}

impl<'a, T> Iter<'a, T> {
    /// Create a new instance of the iterator
    pub fn new(iter: slice::Iter<'a, T>) -> Self {
        Self { iter }
    }
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

impl<T> IntoIter<T> {
    /// Creates a new instance of the into iterator.
    pub fn new(iter: std::vec::IntoIter<T>) -> Self {
        Self { iter }
    }
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
