use yos_collections::binheap::BinHeap;

fn main() {
    let mut h = BinHeap::<u8>::new();

    // h.push(4);
    // h.push(2);
    // h.push(3);
    // h.push(5);
    // println!("heap={:?}", h);

    // let v = h.pop();
    // println!("v={:?} heap={:?}", v, h);

    // let v = h.pop();
    // println!("v={:?} heap={:?}", v, h);

    h.push(4);
    h.push(5);
    h.push(3);
    h.push(11);
    h.push(8);
    h.push(15);

    let v = h.pop();
    println!("v={:?} heap={:?}", v, h);

    let v = h.pop();
    println!("v={:?} heap={:?}", v, h);

    let v = h.pop();
    println!("v={:?} heap={:?}", v, h);

    let v = h.pop();
    println!("v={:?} heap={:?}", v, h);

    let v = h.pop();
    println!("v={:?} heap={:?}", v, h);

    let v = h.pop();
    println!("v={:?} heap={:?}", v, h);

    let v = h.pop();
    println!("v={:?} heap={:?}", v, h);
}
