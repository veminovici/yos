use yos_collections::binomial_heap::*;

fn main() {
    let a = Node::with_order(10, 1);
    let b = Node::with_order(20, 1);

    let mut bxa = Box::new(a);
    let bxb = Box::new(b);

    append(&mut bxa, Some(bxb));

    println!("Now: {:?}", bxa);
}
