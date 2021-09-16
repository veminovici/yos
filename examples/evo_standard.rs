use yos::evolution::standard::*;
use yos::BitstringDebug;

fn show_mutate() {
    let cut = 1;
    println!("Mutating an u8 at position {} ...", cut);
    let mut x = 5u8;
    println!("Before: {}", x.bdebug());
    mutate(&mut x, cut);
    println!("After:  {}", x.bdebug());
    assert_eq!(x, 7);
    println!();
}

fn show_crossover() {
    let cut = 4;
    println!("Crossovering two u8s at position {} ...", cut);
    let mut a = 53u8;
    println!("Before a={}", a.bdebug());
    let mut b = 200u8;
    println!("Before b={}", b.bdebug());

    crossover(&mut a, &mut b, cut);
    println!("After  a={}", a.bdebug());
    println!("Affter b={}", b.bdebug());
    println!();

    crossover(&mut a, &mut b, cut);
    assert_eq!(a, 53u8);
    assert_eq!(b, 200u8);
}

fn main() {
    show_mutate();
    show_crossover();
}
