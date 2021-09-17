use yos_bitstring::{BitstringCombinators, BitstringDebug, BitstringInto, BitstringRange};

fn main() {
    let mut x = 7u8;
    println!("just print it - {}", x.bdebug());

    x.rst_low(2);
    println!("reset the 2 lowest bits - {}", x.bdebug());

    let mut x = 6u8;
    println!("start with - {}", x.bdebug());
    x.flip(1);
    println!("and flip the 1st bit - {}", x.bdebug());
    x.flip(1);
    println!("and flip back the 1st bit - {}", x.bdebug());

    let x = 5u64;
    let bits = x.to_bits();
    println!("bits={:?}", &bits);
}
