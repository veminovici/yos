use yos_binheap::Hole;

#[test]
fn hole_new() {
    let xs = [1, 2, 3, 4, 5];
    let mut ys = [1, 2, 3, 4, 5];
    unsafe {
        let _ = Hole::new(&mut ys, 4);
    }

    assert_eq!(xs, ys);
}
