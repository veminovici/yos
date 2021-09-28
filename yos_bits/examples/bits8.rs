use std::iter::FromIterator;
use yos_bitstring::{Bits8, Bitstring};

fn main() {
    println!("zero={:?}", Bits8::zero());
    for b in Bits8::one() {
        println!("bit={}", b);
    }

    for (ndx, b) in Bits8::from(7u8).into_iter().enumerate() {
        println!("bit {}:{:?}", ndx, b);
    }

    let mut x = Bits8::from(5u8);
    println!("x={:?}", x);

    let y = Bits8::from(3u8);
    println!("y={}", y);

    let z = x & y;
    println!("z={:?}", z);

    println!("0={:?}", Bits8::zero());
    println!("1={:?}", Bits8::one());

    x.set(1);
    println!("x={:?}", x);

    let z = x >> 1;
    println!("z={:?}", z);

    let z1 = x << 1;
    println!("z={:?}", z1);

    x.set_low(1);
    println!("x={:?}", x);

    x.rst_low(2);
    println!("x={:?}", x);

    x.set_high(3);
    println!("x={:?}", x);

    x.rst_high(1);
    println!("x={:?}", x);

    let x = Bits8::range_ones(2, 4);
    println!("x={:?}", x);

    let x = Bits8::range_zeros(1, 6);
    println!("x={:?}", x);

    let x = Bits8::from(2);
    let y = Bits8::from_iter(x.into_iter());
    println!("x={:?}", x);
    println!("y={:?}", y);
}
