use std::mem;

/// The node for the binomial heap
#[derive(Clone)]
pub struct Node<T> {
    /// The stored value
    pub item: T,
    /// The order of the node
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

/// Merges two nodes.
pub fn merge<T>(mut a: &mut Box<Node<T>>, mut b: Box<Node<T>>) {
    loop {
        let a_ = a;
        eprintln!("a_order={} b_order={}", a_.order, b.order);

        // the node 'a' will always be the one with the smallest order value,
        // while the node 'b' will always have the greater order value.
        if a_.order > b.order {
            mem::swap(a_, &mut b);
            eprintln!("swapped a_order={} b_order={}", a_.order, b.order);
        }

        match a_.sibling {
            None =>
            // 'a' does not have any siblings, b will be its sibling.
            {
                return a_.sibling = Some(b)
            }
            Some(ref mut next) =>
            // we continue walking along the chain of siblings
            // the sibling becomes the new 'a'
            {
                a = next
            }
        }

        eprintln!("a_order={}", a_.order);
    }
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
    fn test_merge() {
        let a = Node::with_order(1, 20);
        let b = Node::with_order(2, 10);

        let mut box_a = Box::new(a);
        let box_b = Box::new(b);

        merge(&mut box_a, box_b);
        assert_eq!(2, box_a.item);
        assert_eq!(10, box_a.order);

        let x = box_a.sibling.as_ref().unwrap();
        assert_eq!(1, x.item);
        assert_eq!(20, x.order);
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
