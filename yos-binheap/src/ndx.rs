/// Determines the index of the parent node into a heap vector
/// given the index of the current node and the size of the nodes.
fn heap_parent_ndx(pos: usize, sz: usize) -> usize {
    debug_assert!(sz != 0);

    if pos == 0 {
        0
    } else {
        (pos - 1) / sz
    }
}

/// Determines the index of a chlid node into a heap vector
/// given the index of the current node, the order of the child, and the size of the nodes.
fn heap_child_ndx(pos: usize, c: usize, sz: usize) -> usize {
    pos * sz + c + 1
}

/// Determines if an element is the root of the heap.
#[inline]
pub(super) fn heap_is_root(pos: usize) -> bool {
    pos == 0
}

/// Determine the index of the parent for an element at a given
/// position in a binary heap.
#[inline]
pub(super) fn bheap_parent_ndx(pos: usize) -> usize {
    heap_parent_ndx(pos, 2)
}

/// Determine the index of the child for an element at a given
/// position in a binary heap.
#[inline]
pub(super) fn bheap_child_ndx(pos: usize, c: usize) -> usize {
    heap_child_ndx(pos, c, 2)
}

/// Determines if an element is the root of the heap.
#[inline]
pub(super) fn bheap_is_root(pos: usize) -> bool {
    heap_is_root(pos)
}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn test_heap_parent_ndx_root() {
        let p = heap_parent_ndx(0, 2);
        assert_eq!(p, 0);
    }

    #[test]
    fn test_bheap_parent_ndx_child() {
        assert_eq!(bheap_parent_ndx(1), 0);
        assert_eq!(bheap_parent_ndx(2), 0);
        assert_eq!(bheap_parent_ndx(3), 1);
        assert_eq!(bheap_parent_ndx(4), 1);
        assert_eq!(bheap_parent_ndx(5), 2);
        assert_eq!(bheap_parent_ndx(6), 2);
    }

    #[test]
    fn test_bheap_child_ndx() {
        let c = bheap_child_ndx(0, 0);
        assert_eq!(1, c);

        let c = bheap_child_ndx(0, 1);
        assert_eq!(2, c);

        let c = bheap_child_ndx(1, 0);
        assert_eq!(3, c);

        let c = bheap_child_ndx(1, 1);
        assert_eq!(4, c);

        let c = bheap_child_ndx(2, 0);
        assert_eq!(5, c);

        let c = bheap_child_ndx(2, 1);
        assert_eq!(6, c);
    }

    #[test]
    fn test_bheap_is_root() {
        assert!(bheap_is_root(0));
        assert!(!bheap_is_root(1));
    }
}
