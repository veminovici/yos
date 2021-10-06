use yos_collections::binheap::BinHeap;

fn main() {
    let heap = BinHeap::<u8>::new();
    println!("heap={:?}", heap);
}
