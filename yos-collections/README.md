# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...yos_collections
A rust crate for ddifferent collection data structures.

## BinaryHeap
An implementation of a binary heap. For some details please check the wikipedia [page](https://en.wikipedia.org/wiki/Binary_heap).

| Operations | Average | Worst case
---|---|---
Space | O(n) | O(n)
Search | O(n) | O(n)
Insert | O(1) | O(log n)
Find-min | O(1) | O(1)
Delete-min | O(log n) | O(log n)

#### Example

```rust
use yos_collections::binary_heap::BinaryHeap;

let mut h = BinaryHeap::<u8>::new();
h.push(4);
h.push(2);
h.push(3);
h.push(5);

let v = h.pop().unwrap();
assert_eq!(5, v);
```

