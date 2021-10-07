use yos_collections::binomial_heap::*;

fn main() {
    let a = Node {
        item: 1,
        order: 20,
        sibling: None,
        child: None,
    };

    let b = Node {
        item: 2,
        order: 10,
        sibling: None,
        child: None,
    };

    let mut box_a = Box::new(a);
    let box_b = Box::new(b);

    merge(&mut box_a, box_b);
    eprintln!("a_sibling={}", box_a.sibling.unwrap().order);
}
