use yos_binheap::BinHeap;

fn main() {
    let mut h = BinHeap::new();
    h.push(11);
    //println!("{:?}", h);
    h.iter().for_each(|x| println!("Element: {:02}", x));

    h.push(5);
    h.iter().for_each(|x| println!("Element: {:02}", x));
    //println!("{:?}", h);
    //h.push(8);
    //h.push(3);
    //h.push(4);
    //h.push(15);
}
