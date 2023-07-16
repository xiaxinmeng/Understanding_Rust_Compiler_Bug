
use std::process;

fn main() {
    let r1 = foo(&mut 1, &mut 2, &3);
    let r2 = foo(&mut 1, &mut 2, &4);
    process::exit(r1 + r2)
}

#[inline(never)]
pub fn foo(a: &mut i32, b: &mut i32, x: &i32) -> i32 {
  *a = *x;
  *b = *x;
  *a
}
