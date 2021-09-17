use yos_bitstring::{BitstringDebug, BitstringOps};

fn main() {
    let mut x = 7u8;
    println!("{}", x.bdebug());

    x.rst_low(2);
    println!("{}", x.bdebug());
}
