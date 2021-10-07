use yos_collections::binary_heap::BinaryHeap;

fn main() {
    let h = BinaryHeap::from(vec![1, 2, 3]);
    println!("heap={:?}", h);
}
