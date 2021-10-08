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
}

/// Peek into a max-binomial-tree.
pub fn peek<T: Ord>(root: &Option<Box<Node<T>>>) -> Option<&T> {
    root.as_ref().map(|mut max| {
        // Keep going to the next sibling to find the maximum value among all sibling values.

        // Get the next sibling
        let mut a = &max.sibling;

        while let Some(ref b) = *a {
            // Compare the sibling's value with the current max.
            if b.item > max.item {
                max = b;
            }

            // Go to the next sibling
            a = &b.sibling;
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
                debug_assert!(current.order < b.order);
                return current.sibling = Some(b)
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

/// Remove the max element in the binomial max-heap
pub fn remove_max<T: Ord>(mut a: &mut Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    a.take().map(|mut max| {
        *a = max.sibling.take();

        loop {
            let a_ = a;

            match *a_ {
                None => {
                    return max
                }
                Some(ref mut b) => {
                    if b.item > max.item {
                        max.sibling = b.sibling.take();
                        mem::swap(&mut max, b);
                    }
                }
            }

            a = &mut a_.as_mut().unwrap().sibling;
        }
    })
}

/// Makes `b` a child of `a`.
pub fn add_child<T: Ord>(a: &mut Node<T>, mut b: Box<Node<T>>) {
    debug_assert!(a.order == b.order);
    debug_assert!(b.sibling.is_none());
    debug_assert!(a.item >= b.item);

    b.sibling = a.child.take();
    a.child = Some(b);
    a.order += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek() {
        let n3 = Node::with_order(3, 1);

        let mut n2 = Node::with_order(2, 1);
        n2.sibling = Some(Box::new(n3));

        let mut n1 = Node::with_order(1, 1);
        n1.sibling = Some(Box::new(n2));

        let mut n0 = Node::with_order(0, 1);
        n0.sibling = Some(Box::new(n1));

        let x = Some(Box::new(n0));
        let v = peek(&x).unwrap();
        assert_eq!(3, *v);
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
    fn test_add_child() {
        let mut a = Node::with_order(20, 1);
        let b = Node::with_order(10, 1);
        let box_b = Box::new(b);

        add_child(&mut a, box_b);

        assert_eq!(20, a.item);
        assert_eq!(2, a.order);
        assert_eq!(10, a.child.as_ref().unwrap().item);
        assert_eq!(1, a.child.as_ref().unwrap().order);
    }
}
