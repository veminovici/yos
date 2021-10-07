use yos_collections::binheap::BinHeap;

fn main() {
    let mut heap = BinHeap::<u8>::new();

    heap.push(4);
    heap.push(5);
    heap.push(8);
    heap.push(11);
    heap.push(3);
    println!("heap={:?}", heap);

    heap.push(15);
    println!("heap={:?}", heap);

    let v = heap.pop();
    println!("v={:?} heap={:?}", v, heap);
}
