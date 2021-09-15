use yos::*;

fn main() {
    let v = 5u8;
    println!("v={}", v.debug());

    //let us = u8s(v);
    //println!("u8s={:?}", us);

    let v = 5u64;
    println!("v={}", v.debug());

    //let u: u8 = 0b11111111u8;
    //println!("u={}", u);

    //let val = 0u8;
    //let ueights = val.ueights();
    //println!("{:?}", ueights);
}
