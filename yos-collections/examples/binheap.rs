use yos_collections::binheap::BinHeap;

fn main() {
    let h = BinHeap::from(vec![1, 2, 3]);
    println!("heap={:?}", h);
}
