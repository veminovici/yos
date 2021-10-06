use core::ptr;
use std::mem::ManuallyDrop;

/// MemCursor represents a cursor in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `MemCursor` will restore the slice by filling the hole
/// position with the value that was originally removed.
pub struct MemCursor<'a, T: 'a> {
    data: &'a mut [T],
    elt: ManuallyDrop<T>,
    pos: usize,
}

impl<'a, T> MemCursor<'a, T> {
    /// Creates a new cursor over a slice of memory.
    ///
    /// # Example
    ///
    /// ```
    /// use yos_memcursor::MemCursor;
    ///
    /// let mut xs = [1, 2, 3, 4, 5];
    /// unsafe {
    ///     let mut mc = MemCursor::new(&mut xs, 4);
    ///     mc.move_to(1);
    /// }
    ///
    /// assert_eq!([1, 5, 3, 4, 2], xs);
    /// ```
    ///
    /// # Safety
    /// The position must be within the length of the memory slice.
    #[inline]
    pub unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
        debug_assert!(pos < data.len());

        // SAFE: pos should be inside the slice
        let elt = ptr::read(data.get_unchecked(pos));
        Self {
            data,
            elt: ManuallyDrop::new(elt),
            pos,
        }
    }

    /// Returns the position of the cursor.
    ///
    /// # Example
    ///
    /// ```
    /// use yos_memcursor::MemCursor;
    ///
    /// let mut xs = [1, 2, 3, 4, 5];
    /// unsafe {
    ///     let mut mc = MemCursor::new(&mut xs, 4);
    ///     mc.move_to(1);
    ///     assert_eq!(1, mc.pos());
    /// }
    ///
    /// assert_eq!([1, 5, 3, 4, 2], xs);
    /// ```
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// The element that could fill in the cursor position.
    ///
    /// # Example
    ///
    /// ```
    /// use yos_memcursor::MemCursor;
    ///
    /// let mut xs = [1, 2, 3, 4, 5];
    /// unsafe {
    ///     let mut mc = MemCursor::new(&mut xs, 4);
    ///     mc.move_to(1);
    ///     assert_eq!(5, *mc.element());
    /// }
    ///
    /// assert_eq!([1, 5, 3, 4, 2], xs);
    /// ```
    #[inline]
    pub fn element(&self) -> &T {
        &self.elt
    }

    /// Returns a reference to the element at `index`.
    /// Unsafe because index must be within the data slice and not equal to pos.
    ///
    /// # Example
    ///
    /// ```
    /// use yos_memcursor::MemCursor;
    ///
    /// let mut xs = [1, 2, 3, 4, 5];
    /// unsafe {
    ///     let mut mc = MemCursor::new(&mut xs, 4);
    ///     mc.move_to(1);
    ///     assert_eq!(3, *mc.get(2));
    /// }
    ///
    /// assert_eq!([1, 5, 3, 4, 2], xs);
    /// ```
    ///
    /// # Safety
    /// The index must be within the data slice and different than the current position.
    #[inline]
    pub unsafe fn get(&self, index: usize) -> &T {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());

        self.data.get_unchecked(index)
    }

    /// Move cursor to new location
    /// Unsafe because index must be within the data slice and not equal to pos.
    ///
    /// # Example
    ///
    /// ```
    /// use yos_memcursor::MemCursor;
    ///
    /// let mut xs = [1, 2, 3, 4, 5];
    /// unsafe {
    ///     let mut mc = MemCursor::new(&mut xs, 4);
    ///     mc.move_to(2);
    ///     mc.move_to(1);
    /// }
    ///
    /// assert_eq!([1, 5, 2, 4, 3], xs);
    /// ```
    ///
    /// # Safety
    /// The index must be withing the data slice and different than the current position.
    #[inline]
    pub unsafe fn move_to(&mut self, index: usize) {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());

        let index_ptr: *const _ = self.data.get_unchecked(index);
        let hole_ptr = self.data.get_unchecked_mut(self.pos);

        ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);

        self.pos = index;
    }
}

impl<T> Drop for MemCursor<'_, T> {
    #[inline]
    fn drop(&mut self) {
        // fill the hole again when the current cursor position runs our of scope.
        unsafe {
            let pos = self.pos;
            ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_pass() {
        let mut xs = [1, 2, 3, 4, 5];
        unsafe {
            let _ = MemCursor::new(&mut xs, 4);
        }

        assert_eq!([1, 2, 3, 4, 5], xs);
    }

    #[test]
    fn move_to_pass() {
        let mut xs = [1, 2, 3, 4, 5];

        unsafe {
            let mut mc = MemCursor::new(&mut xs, 4);
            mc.move_to(1);
        }

        assert_eq!([1, 5, 3, 4, 2], xs);
    }

    #[test]
    fn pos_pass() {
        let mut xs = [1, 2, 3, 4, 5];

        unsafe {
            let mut mc = MemCursor::new(&mut xs, 4);
            mc.move_to(1);
            assert_eq!(1, mc.pos());
        }
    }

    #[test]
    fn pos_element() {
        let mut xs = [1, 2, 3, 4, 5];

        unsafe {
            let mut mc = MemCursor::new(&mut xs, 4);
            mc.move_to(1);
            assert_eq!(5, *mc.element());
        }
    }

    #[test]
    fn pos_get() {
        let mut xs = [1, 2, 3, 4, 5];

        unsafe {
            let mut mc = MemCursor::new(&mut xs, 4);
            mc.move_to(1);
            assert_eq!(2, *mc.get(4));
        }
    }
}
