rust
#![feature(generators)]

fn f() {
    || yield ((1,1),);
}
