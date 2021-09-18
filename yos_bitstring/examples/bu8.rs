use yos_bitstring::{Bitstring, Bu8};

fn main() {
    let mut x = Bu8::from(5u8);
    println!("x={:?}", x);

    let y = Bu8::from(3u8);
    println!("y={}", y);

    let z = x & y;
    println!("z={:?}", z);

    println!("0={:?}", Bu8::zero());
    println!("1={:?}", Bu8::one());

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
}
