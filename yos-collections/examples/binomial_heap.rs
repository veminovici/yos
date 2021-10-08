use yos_collections::binomial_heap::*;

fn main() {
    // The nodes are represented: (order, item)
    // Build a chain (1, 10) -> (2, 20) -> (2, 30) -> (4, 40)
    let d = Node::with_order(40, 4);

    let mut c = Node::with_order(20, 2);
    c.set_sibling(d);

    let mut b = Node::with_order(30, 2);
    b.set_sibling(c);

    let mut a = Node::with_order(10, 1);
    a.set_sibling(b);

    let mut box_a = Box::new(a);
    coalesce(&mut box_a);

    assert_eq!(10, box_a.item);
    assert_eq!(1, box_a.order);

    let box_x = box_a.sibling.as_ref().unwrap();
    assert_eq!(30, box_x.item);
    assert_eq!(3, box_x.order);
}
