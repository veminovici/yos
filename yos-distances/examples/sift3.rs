use yos_distances::sift3;

fn main() {
    let d = sift3("hannah".as_bytes(), "ahanna".as_bytes(), 5);
    println!("d={}", d);
}
