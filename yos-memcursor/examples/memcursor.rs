use yos_memcursor::MemCursor;

fn main() {
    let mut xs = [1, 2, 3, 4, 5];

    println!("before: {:?}", &xs);
    unsafe {
        let mut mc = MemCursor::new(&mut xs, 4);
        mc.move_to(2);
        assert_eq!(*mc.get(4), 3);

        mc.move_to(0);
        assert_eq!(*mc.get(2), 1);
    }
    println!("after: {:?}", &xs);
    assert_eq!([5, 2, 1, 4, 3], xs)
}
