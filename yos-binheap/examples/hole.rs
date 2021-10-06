use yos_binheap::Hole;

fn main() {
    let mut xs = [1, 2, 3, 4, 5];

    println!("before: {:?}", &xs);
    unsafe {
        let mut hole = Hole::new(&mut xs, 4);
        //hole.move_to(3);
        hole.move_to(2);
        hole.move_to(0);
    }

    println!("after: {:?}", &xs);
}
