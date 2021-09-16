use yos::BitstringDebug;

mod common;

#[test]
fn it_test() {
    common::init();
    let x = 5u8;
    println!("x={}", x.bdebug());
}
