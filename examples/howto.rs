use yos::{Bitstring, BitstringDebug};

fn main() {
    // for i in 0..9 {
    //     println!("low_mask_{}: {}", i, u8::low_mask(i).debug());
    // }

    // for i in 0..9 {
    //     println!("high_mask_{}: {}", i, u8::high_mask(i).debug());
    // }

    // for i in 0..65 {
    //     println!("low_mask_{:2}: {}", i, u64::low_mask(i).debug());
    // }

    // for i in 0..65 {
    //     println!("high_mask_{:2}: {}", i, u64::high_mask(i).debug());
    // }

    let x = 56u8;
    println!("x={}", x.bdebug());

    let (h, t) = x.bsplit(0);
    println!("h0={}", h.bdebug());
    println!("t0={}", t.bdebug());

    let (h, t) = x.bsplit(4);
    println!("h4={}", h.bdebug());
    println!("t4={}", t.bdebug());

    let (h, t) = x.bsplit(8);
    println!("h4={}", h.bdebug());
    println!("t4={}", t.bdebug());
}
