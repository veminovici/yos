# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Yos... 

Just another rust crate, this one implements some bit-strings.

[![ci pipeline](https://github.com/veminovici/yos/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/veminovici/yos/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/veminovici/yos/badge.svg)](https://coveralls.io/github/veminovici/yos)
[![Last commit](https://img.shields.io/github/last-commit/veminovici/yos)](https://github.com/veminovici/yos)
[![Repo size](https://img.shields.io/github/repo-size/veminovici/yos)](https://github.com/veminovici/yos)

[![Github Actions](https://buildstats.info/github/chart/veminovici/yos)](https://github.com/veminovici/yos)

</br>

### Bitstring & BitstrigDebug Traits
The **Bitstring** trait defines the behavior that control a sequence of bits, such *OR*, *AND*, *XOR* bitewise operations. For a complete list of functions, see the [source](https://github.com/veminovici/yos/blob/main/src/traits.rs) code.

```rust
pub trait Bitstring {
    /// AND bitwise operation between two bitstrings
    fn band(&mut self, other: &Self);

    /// OR bitwise operation between two bitstrings
    fn bor(&mut self, other: &Self);

    // many more ...
}
```

The **BitstringDebug** trait defines the function **bdebug** that prints a bit-string into a friendly format.

</br>

### Extending u8, u64
The crate extends the functionality of **u8**, and **u64** primitive data types by implementing the **Bitstring** and **BitstringDebig** traits on those data types.

```rust
use yos::{Bitstring, BitstringDebug};

let v = 56u8;
println!("v={}", v.debug()); // u8: 56|00111000|
```


</br>

### Thank you!!!

> You can contact me at veminovici@hotmail.com. Code designed and written in Päädu, on the beautiful island of [**Saaremaa**](https://goo.gl/maps/DmB9ewY2R3sPGFnTA), Estonia.
