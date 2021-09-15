use yos::*;

fn main() {
    let v = 5u8;
    println!("v={}", v.debug());

    //let us = u8s(v);
    //println!("u8s={:?}", us);

    let v = 5u64;
    println!("v={}", v.debug());

    for i in 0..9 {
        println!("low_mask_{}: {}", i, u8::low_mask(i).debug());
    }

    for i in 0..9 {
        println!("high_mask_{}: {}", i, u8::high_mask(i).debug());
    }

    for i in 0..65 {
        println!("low_mask_{:2}: {}", i, u64::low_mask(i).debug());
    }

    for i in 0..65 {
        println!("high_mask_{:2}: {}", i, u64::high_mask(i).debug());
    }
}
