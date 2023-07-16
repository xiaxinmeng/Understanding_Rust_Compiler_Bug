rust
#![feature(bind_by_move_pattern_guards)]

#[derive(Debug)]
struct A;

impl A {
    fn foo(&self) -> bool { dbg!(2); false }
}

impl Drop for A {
    fn drop(&mut self) { dbg!(3); }
}

fn main() {
    dbg!(0);
    match dbg!(A) {
        a if a.foo() => { dbg!(4); }
        _ => { dbg!(5); }
    }
    dbg!(6);
}
