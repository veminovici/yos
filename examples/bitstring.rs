use yos::{Bitstring, BitstringDebug};

fn main() {
    println!("Declare an u8 as a bitstring:");
    let x = 56u8;
    println!("x={}", x.bdebug());
    assert_eq!(x, 56);
    println!();

    println!("Split an u8 into head and tail at different cutting points:");
    let cut = 0;
    let (h, t) = x.bsplit(cut);
    println!("cut={}, head={}, tail={}", cut, h.bdebug(), t.bdebug());
    assert_eq!(h, 0);
    assert_eq!(t, 56);

    let cut = 4;
    let (h, t) = x.bsplit(cut);
    println!("cut={}, head={}, tail={}", cut, h.bdebug(), t.bdebug());
    assert_eq!(h, 8);
    assert_eq!(t, 48);

    let cut = 8;
    let (h, t) = x.bsplit(cut);
    println!("cut={}, head={}, tail={}", cut, h.bdebug(), t.bdebug());
    assert_eq!(h, 56);
    assert_eq!(t, 0);

    println!();

    println!("Print all the low one-masks for u8");
    for i in 0..9 {
        println!("u8 low_len={}: {}", i, u8::bone_low(i).bdebug());
    }
    println!();

    println!("Print all the high one-masks for u8");
    for i in 0..9 {
        println!("high_len={}: {}", i, u8::bone_high(i).bdebug());
    }
    println!();

    println!("Print all the low one-masks for u64");
    for i in 0..65 {
        println!("low_len={:2}: {}", i, u64::bone_low(i).bdebug());
    }
    println!();

    println!("Print all the high one-masks for u64");
    for i in 0..65 {
        println!("high_len={:2}: {}", i, u64::bone_high(i).bdebug());
    }
    println!();
}
