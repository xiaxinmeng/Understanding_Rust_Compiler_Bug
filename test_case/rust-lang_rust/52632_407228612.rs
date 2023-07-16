rust
#![feature(existential_type)]

existential type Clonable: Clone;

fn main() {
    let _x: Clonable = 0i32;
}
