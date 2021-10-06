use core::ptr;
use std::mem::ManuallyDrop;

/// Hole represents a hole in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
pub struct Hole<'a, T: 'a> {
    data: &'a mut [T],
    elt: ManuallyDrop<T>,
    pos: usize,
}

impl<'a, T> Hole<'a, T> {
    /// Creates a new hole into a slice of memory.
    ///
    /// # Safety
    /// The position must be within the length of the memory slice.
    #[inline]
    pub unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
        debug_assert!(pos < data.len());

        // SAFE: pos should be inside the slice
        let elt = ptr::read(data.get_unchecked(pos));
        Hole {
            data,
            elt: ManuallyDrop::new(elt),
            pos,
        }
    }

    /// Returns the position of the hole.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// The element that could fill in the hole.
    #[inline]
    pub fn element(&self) -> &T {
        &self.elt
    }

    /// Returns a reference to the element at `index`.
    /// Unsafe because index must be within the data slice and not equal to pos.
    ///
    /// # Safety
    /// The index must be within the data slice and different than the current position.
    #[inline]
    pub unsafe fn get(&self, index: usize) -> &T {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());

        self.data.get_unchecked(index)
    }

    /// Move hole to new location
    /// Unsafe because index must be within the data slice and not equal to pos.
    ///
    /// # Safety
    /// The index must be withing the data slice and different than the current position.
    #[inline]
    pub unsafe fn move_to(&mut self, index: usize) {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());

        // eprintln!("Hole::move_to {}..>{}, new pos={}", self.pos, index, index);

        let index_ptr: *const _ = self.data.get_unchecked(index);
        let hole_ptr = self.data.get_unchecked_mut(self.pos);

        ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);

        self.pos = index;
    }
}

impl<T> Drop for Hole<'_, T> {
    #[inline]
    fn drop(&mut self) {
        // fill the hole again when the hole instance runs our of scope.
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
            let _ = Hole::new(&mut xs, 4);
        }

        assert_eq!([1, 2, 3, 4, 5], xs);
    }

    #[test]
    fn move_to_pass() {
        let mut xs = [1, 2, 3, 4, 5];

        unsafe {
            let mut h = Hole::new(&mut xs, 4);
            h.move_to(1);
        }

        assert_eq!([1, 5, 3, 4, 2], xs);
    }
}
