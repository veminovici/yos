# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee.Yos 

Just another rust crate.

[![ci pipeline](https://github.com/veminovici/yos/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/yos/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/veminovici/yos/badge.svg)](https://coveralls.io/github/veminovici/yos)
[![Last commit](https://img.shields.io/github/last-commit/veminovici/yos)](https://github.com/veminovici/yos)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/yos)](https://github.com/veminovici/yos)

[![Github Actions](https://buildstats.info/github/chart/veminovici/yos)](https://github.com/veminovici/yos)

### Bitwise Trait
The **Bitwise** trait defines the behavior that control a sequence of bits.

```rust
pub trait Bitwise {
    /// Set the bit to a given index
    fn set(&mut self, ndx: usize);

    /// Reset the bit to a given index
    fn reset(&mut self, ndx: usize);

    /// Flip the value for a bit at a given index
    fn flip(&mut self, ndx: usize);

    /// Returns the value of a bit at a given index
    fn get(&self, ndx: usize) -> u8;

    /// Reset the low indexed bits
    fn reset_low(&mut self, n: usize);

    /// Reset the high indexed bits
    fn reset_high(&mut self, n: usize);

    /// Returns the list of u8 values.
    fn ueights(&self) -> Vec<u8>;
}
```

### Extending u8, u64
The crate extends the functionality of **u8**, and **u64** primitive data types by implementing the **Bitewise** trait on those data types.

```rust
use yos::*;

let mut v = 5u8;
v.reset(1);
assert_eq!(v, 4);
println!("v={}", v.debug());
```

### Thank you!!!

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.
