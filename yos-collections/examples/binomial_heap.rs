use yos_collections::binomial_heap::*;

fn main() {
    let mut a = Node::with_order(10, 1);
    let a1 = Node::with_order(9, 0);
    a.set_child(a1);

    let mut b = Node::with_order(20, 1);
    let b1 = Node::with_order(19, 0);
    b.set_child(b1);

    let mut bxa = Box::new(a);
    let bxb = Box::new(b);

    append(&mut bxa, Some(bxb));

    assert_eq!(bxa.item, 20);
    assert_eq!(bxa.order, 2);

    let bxx = bxa.child.as_ref().unwrap();
    assert_eq!(bxx.item, 19);
    assert_eq!(bxx.order, 0);

    let bxy = bxx.sibling.as_ref().unwrap();
    assert_eq!(bxy.item, 10);
    assert_eq!(bxy.order, 1);
}
