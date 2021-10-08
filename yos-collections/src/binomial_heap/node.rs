use std::fmt::Debug;
use std::mem;

/// The node for the binomial heap
#[derive(Clone)]
pub struct Node<T> {
    /// The stored value.
    pub item: T,
    /// The order of the node, we have 2^order elements
    /// in the tree with the root in this node.
    pub order: usize,
    /// Optional sibling node
    pub sibling: Option<Box<Node<T>>>,
    /// Optional child node
    pub child: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// Creates a new instance of a node.
    pub fn with_order(item: T, order: usize) -> Self {
        Node {
            item,
            order,
            sibling: None,
            child: None,
        }
    }

    /// Set the sibling node.
    pub fn set_sibling(&mut self, sibling: Node<T>) {
        self.sibling = Some(Box::new(sibling))
    }

    /// Set the child node.
    pub fn set_child(&mut self, child: Node<T>) {
        self.child = Some(Box::new(child))
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .sibling
            .as_ref()
            .map_or("_".to_string(), |s| format!("{:?}", s.item));
        let c = self
            .child
            .as_ref()
            .map_or("_".to_string(), |c| format!("{:?}", c.item));

        write!(f, "o{}:v{:?}:s{}:c{}", self.order, self.item, s, c)
    }
}

/// Peek into the max-heap. Each of the nodes in the chain of siblings
/// is a max-heap, so we just need to walk the chain and find the node
/// with the max value.
pub fn peek<T: Ord>(root: &Option<Box<Node<T>>>) -> Option<&T> {
    root.as_ref().map(|mut max| {
        // Get the next sibling
        let mut sibling_o = &max.sibling;

        while let Some(ref current) = *sibling_o {
            // Compare the sibling's value with the current max.
            if current.item > max.item {
                max = current;
            }

            // Go to the next sibling
            sibling_o = &current.sibling;
        }

        &max.item
    })
}

/// Merges two nodes. We are operating within the sibling chain, trying
/// to insert 'b' into the chain in such way that we preserve the order.
pub fn merge<T>(mut a: &mut Box<Node<T>>, mut b: Box<Node<T>>) {
    loop {
        let current = a;

        // we try to presenr the order, so if 'b' is smaller than 'current'
        // we will swap the two nodes.
        if current.order > b.order {
            mem::swap(current, &mut b);
        }

        match current.sibling {
            None =>
            // 'current' does not have any siblings, add 'b' as its sibling.
            {
                return current.sibling = Some(b);
            }
            Some(ref mut sibling) =>
            // 'current' have a sibling, we keep walking in the chain
            // by going to the sibling.
            {
                a = sibling
            }
        }
    }
}

fn link<T: Ord>(a: &mut Node<T>, mut b: Box<Node<T>>) {
    debug_assert!(a.order == b.order);
    debug_assert!(b.sibling.is_none());
    debug_assert!(a.item >= b.item);

    b.sibling = a.child.take();
    a.child = Some(b);
    a.order += 1;
}

/// Coalesce any two nodes in the siblings chain such that we
/// retors the binomial property of the heap.
pub fn coalesce<T: Ord>(mut a: &mut Box<Node<T>>) {
    loop {
        //let current = a;
        match a.sibling {
            None => {
                return;
            }
            Some(ref sibling) => {
                if a.order != sibling.order
                    || sibling
                        .sibling
                        .as_ref()
                        .map_or(false, |c| c.order == sibling.order)
                {
                    // Keep moving

                    // Nothing to do here, the nodes have different orders, so they don't need to be coalesce.
                    // If the 'a' has the same order with 'b' and 'b' has the same order with 'c' we will
                    // go to the 'b' and coalesce it with 'c' which will increment the order of the new 'b'.
                    a = a.sibling.as_mut().unwrap();
                } else if a.item >= sibling.item {
                    // Coalesce under current

                    // The 'a' has the same order value as its sibling.
                    // We need to coalesce the two nodes in order to restore the binomial property.
                    // The coalesce will take place under 'a' since its value is greater than the sibling's one.

                    // get the sibling out of the current node. This sets to None the internal sibling field in 'a'.
                    let mut b = a.sibling.take().unwrap();

                    // 'a' sibling will be 'sibling's sibling.
                    a.sibling = b.sibling.take();

                    // Move the sibling under 'a'
                    link(a, b);
                } else {
                    // Coalesce under sibling

                    // The 'a' has the same order value as its sibling.
                    // We need to coalesce the two nodes in order to restore the binomial property.
                    // The coalesce will take place under 'sibling' since its value is greater than the 'a's one.

                    // Get the sibling out of the current node. This sets to None the internal sibling field in 'a'.
                    let mut b = a.sibling.take().unwrap();

                    // Swap the 'a' with 'sibling'
                    mem::swap(a, &mut b);

                    // Move the sibling, former 'a', under 'a' (which used to be the sibling)
                    link(a, b);
                }
            }
        }
    }
}

/// Apends to the root node another node.
pub fn append<T: Ord>(root: &mut Box<Node<T>>, other: Option<Box<Node<T>>>) {
    if let Some(other) = other {
        merge(root, other);
        coalesce(root);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let n = Node::with_order(0, 0);
        let s = format!("{:?}", n);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_peek_last() {
        // build the chain of siblings
        let n3 = Node::with_order(3, 3);
        let mut n2 = Node::with_order(20, 2);
        n2.sibling = Some(Box::new(n3));

        let mut n1 = Node::with_order(1, 1);
        n1.sibling = Some(Box::new(n2));

        let mut n0 = Node::with_order(0, 0);
        n0.sibling = Some(Box::new(n1));

        let x = Some(Box::new(n0));
        let v = peek(&x).unwrap();
        assert_eq!(20, *v);
    }

    #[test]
    fn test_merge_0_with_1() {
        let a = Node::with_order(1, 0);
        let b = Node::with_order(2, 1);

        let mut box_a = Box::new(a);
        let box_b = Box::new(b);

        merge(&mut box_a, box_b);

        assert_eq!(1, box_a.item);
        assert_eq!(0, box_a.order);

        let box_x = box_a.sibling.as_ref().unwrap();
        assert_eq!(2, box_x.item);
        assert_eq!(1, box_x.order);
    }

    #[test]
    fn test_merge_1_with_0() {
        let a = Node::with_order(2, 1);
        let b = Node::with_order(1, 0);

        let mut box_a = Box::new(a);
        let box_b = Box::new(b);

        merge(&mut box_a, box_b);

        assert_eq!(1, box_a.item);
        assert_eq!(0, box_a.order);

        let box_x = box_a.sibling.as_ref().unwrap();
        assert_eq!(2, box_x.item);
        assert_eq!(1, box_x.order);
    }

    #[test]
    fn test_merge_02_with_13() {
        let mut a = Node::with_order(0, 0);
        let b = Node::with_order(2, 2);
        a.sibling = Some(Box::new(b));

        let mut c = Node::with_order(1, 1);
        let d = Node::with_order(3, 3);
        c.sibling = Some(Box::new(d));

        let mut box_a = Box::new(a);
        let box_c = Box::new(c);
        merge(&mut box_a, box_c);

        assert_eq!(0, box_a.item);
        assert_eq!(0, box_a.order);

        let box_1 = box_a.sibling.as_ref().unwrap();
        assert_eq!(1, box_1.item);
        assert_eq!(1, box_1.order);

        let box_2 = box_1.sibling.as_ref().unwrap();
        assert_eq!(2, box_2.item);
        assert_eq!(2, box_2.order);

        let box_3 = box_2.sibling.as_ref().unwrap();
        assert_eq!(3, box_3.item);
        assert_eq!(3, box_3.order);

        assert!(box_3.sibling.is_none());
    }

    #[test]
    fn test_coalesce_undercurrent() {
        let c = Node::with_order(15, 3);

        let mut b = Node::with_order(10, 0);
        b.sibling = Some(Box::new(c));

        let mut a = Node::with_order(20, 0);
        a.sibling = Some(Box::new(b));

        let mut box_a = Box::new(a);
        coalesce(&mut box_a);

        assert_eq!(20, box_a.item);
        assert_eq!(1, box_a.order);

        let box_x = box_a.sibling.as_ref().unwrap();
        assert_eq!(15, box_x.item);
        assert_eq!(3, box_x.order);
    }

    #[test]
    fn test_coalesce_undersibling() {
        let c = Node::with_order(15, 3);

        let mut b = Node::with_order(20, 0);
        b.sibling = Some(Box::new(c));

        let mut a = Node::with_order(10, 0);
        a.sibling = Some(Box::new(b));

        let mut box_a = Box::new(a);
        coalesce(&mut box_a);

        assert_eq!(20, box_a.item);
        assert_eq!(1, box_a.order);

        let box_x = box_a.sibling.as_ref().unwrap();
        assert_eq!(15, box_x.item);
        assert_eq!(3, box_x.order);
    }

    #[test]
    fn test_coalesce_movenext() {
        let n7 = Node::with_order(70, 4);

        let mut n6 = Node::with_order(60, 3);
        n6.set_sibling(n7);

        let mut n5 = Node::with_order(50, 2);
        n5.set_sibling(n6);

        let mut n4 = Node::with_order(40, 2);
        n4.set_sibling(n5);

        let mut n3 = Node::with_order(30, 2);
        n3.set_sibling(n4);

        let mut n2 = Node::with_order(20, 1);
        n2.set_sibling(n3);

        let mut n1 = Node::with_order(10, 0);
        n1.set_sibling(n2);

        let mut bx = Box::new(n1);
        coalesce(&mut bx);

        assert_eq!(bx.item, 10);
        assert_eq!(bx.order, 0);

        let r2 = bx.sibling.as_ref().unwrap();
        assert_eq!(r2.item, 20);
        assert_eq!(r2.order, 1);

        let r3 = r2.sibling.as_ref().unwrap();
        assert_eq!(r3.item, 30);
        assert_eq!(r3.order, 2);

        let r4 = r3.sibling.as_ref().unwrap();
        assert_eq!(r4.item, 70);
        assert_eq!(r4.order, 5);
    }

    #[test]
    fn test_append() {
        let a = Node::with_order(10, 1);
        let b = Node::with_order(20, 1);

        let mut bxa = Box::new(a);
        let bxb = Box::new(b);

        append(&mut bxa, Some(bxb));

        assert_eq!(bxa.item, 20);
        assert_eq!(bxa.order, 2);

        let bxx = bxa.child.as_ref().unwrap();
        assert_eq!(bxx.item, 10);
        assert_eq!(bxx.order, 1);
    }

    #[test]
    fn test_append_1() {
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
        assert_eq!(bxx.item, 10);
        assert_eq!(bxx.order, 1);

        let bxy = bxx.sibling.as_ref().unwrap();
        assert_eq!(bxy.item, 19);
        assert_eq!(bxy.order, 0);
    }

}
