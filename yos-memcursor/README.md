# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...yos_memcursor
A rust crate for moving around values within a slide of memory.

## Example
In the below example, we have a slice of memory of **i32** values.

 - We create a cursor starting at index 4, the cursor will store the value 5. 
 - Move the cursor to index 2. The cursor moves the value at the index 2 to the original position of the cursor. Once the move is done, the cursor is moved to its new position at index 2.
 - Move the cursor again, this time to index 0. The cursor moves the value 1 to the cursor position, which is index 2. 
 - When the cursor goes out of scope, it updates the value at the final cursor position, index 0, with the stored value, 5.

```rust
use yos_memcursor::MemCursor;

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
