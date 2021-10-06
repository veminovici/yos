# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee.Yos.Cursor
A rust crate for moving around values within a slide of memory.

## Example
In the below example, we have a slice of memory of 5 i32 values. We create a cursor that start at position 4 which means the cursor will store internally the value 5. By moving to position 2, we move the value at the index 2 to the original position of the cursor. Once the move it done, the cursor is moved to index 2. Next we move again the cursor to position 0, which means we move the value 1 to the cursor psotion, which is index 2. In the end, when the cursor goes out of scope, it updates the value at the final cursor position with the stored value.

```rust
let mut xs = [1, 2, 3, 4, 5];

unsafe {
    let mut mc = MemCursor::new(&mut xs, 4);

    mc.move_to(2);
    assert_eq!(*mc.get(4), 3);

    mc.move_to(0);
    assert_eq!(*mc.get(2), 1);
}

assert_eq!([5, 2, 1, 4, 3], xs)
```
