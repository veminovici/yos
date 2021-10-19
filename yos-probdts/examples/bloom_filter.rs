extern crate getrandom;

use getrandom::getrandom;
use yos_probdts::bloom::*;

fn main() {
    let mut filter = Filter::new(10, 100);

    let mut item = vec![0u8, 16];
    getrandom(&mut item).unwrap();

    filter.set(&item);
    assert!(filter.check(&item));
}
