use yos_bitstring::Bit;

fn main() {
    let a = Bit::One;
    let b = Bit::Zero;

    let c = a & b;
    println!("c={}", c);
}
