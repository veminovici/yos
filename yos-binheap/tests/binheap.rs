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

#[test]
fn test_binheap_debug() {
    let h = BinHeap::<u8>::new();
    let s = format!("{:?}", h);
    assert!(!s.is_empty());
}

#[test]
fn test_binheap_push() {
    let mut h = BinHeap::new();
    h.push(10);
    assert_eq!(1, h.len());
}

#[test]
fn test_binheap_peek() {
    let mut h = BinHeap::new();
    let v = h.peek();
    assert_eq!(v, None);

    h.push(10);
    let v = h.peek();
    assert_eq!(v, Some(10));
}
