use yos_binheap::BinHeap;

fn main() {
    let mut h = BinHeap::new();
    h.push(11);
    h.push(5);
    h.push(8);
    h.push(3);
    h.push(4);

    println!("before: {:?}", h);
    h.push(15);
    println!("after: {:?}", h);

    //let v = h.peek();
    //println!("head: {:?}", v);
}
