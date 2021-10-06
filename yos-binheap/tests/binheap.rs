use yos_binheap::BinHeap;

#[test]
fn test_binheap_default() {
    let h = BinHeap::<u8>::default();
    assert_eq!(h.len(), 0);
    assert!(h.is_empty());
}

#[test]
fn test_binheap_new() {
    let h = BinHeap::<u8>::new();
    assert_eq!(h.len(), 0);
    assert!(h.is_empty());
}
