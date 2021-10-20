extern crate getrandom;

use getrandom::getrandom;
use yos_probdts::bloom::*;

fn main() {
    let khasher = K2Hasher::new();

    let mut item = vec![0u8, 16];
    getrandom(&mut item).unwrap();

    println!("Round 1");
    let hash_iter = khasher.iter(&item);
    hash_iter
        .take(10)
        .enumerate()
        .for_each(|(i, h)| println!("{}, {}", i, h));
}
