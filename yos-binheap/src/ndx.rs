/// Determines the index of the parent node into a heap vector
/// given the index of the current node and the size of the nodes.
pub(super) fn parent_ndx(pos: usize, sz: usize) -> usize {
    debug_assert!(sz != 0);

    if pos == 0 {
        0
    } else {
        (pos - 1) / sz
    }
}

/// Determine the index of the parent for an element at a given
/// position in a binary heap.
#[inline]
pub(super) fn bheap_parent_ndx(pos: usize) -> usize {
    parent_ndx(pos, 2)
}
