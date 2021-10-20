# ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) Simplee...Yos...Probdts...
A rust create for probabilistic data types.

## K2Hasher
This structure allows you generating *k* hashes for a given *item*. The item can be of any type that implements the [Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html) trait. The hashers uses internally two [SipHasher](https://doc.rust-lang.org/std/hash/struct.SipHasher.html) instances which are initialized using a given seed.The hasher exposes an iterator which lets the caller get the next hash value. The ahsher can genrate an infinite number of hash values, the iterator never ends. In the example below, the hasher is initialized using a randomly generated seed. It generates *10* has values for a given item which in the example is a vector of *16* *u8* values.

```rust
extern crate getrandom;

use getrandom::getrandom;
use yos_probdts::bloom::*;

let khasher = K2Hasher::new();

let mut item = vec![0u8, 16];
getrandom(&mut item).unwrap();

let hash_iter = khasher.iter(&item);
hash_iter.take(10).enumerate().for_each(|(i, h)| println!("{}, {}", i, h));
```

## Bloom-Filter

## Counting-Bloom-Filter

## Resources

- [SipHash](https://en.wikipedia.org/wiki/SipHash)
- [Bloom Filter](https://en.wikipedia.org/wiki/Bloom_filter)
- [Counting Bloom Filter](https://en.wikipedia.org/wiki/Counting_Bloom_filter)
