rust
#[macro_use]
extern crate log;

fn do_stuff() -> bool {
    println!("println");
    error!("error");
    true
}

#[test]
fn my_test() {
    assert_eq!(true, do_stuff());
}
