use yos_bytes::DifficultHash;

fn main() {
    let x = format!("{:02X?}", 1u8);
    let y = format!("{:02X?}", 2u8);
    let z = x + &y;
    println!("z={}", z);

    let xs: Vec<u8> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let h = DifficultHash::new(xs, 16);
    println!("DHash: {}", &h);
    println!("DHash: {:?}", &h);
}
