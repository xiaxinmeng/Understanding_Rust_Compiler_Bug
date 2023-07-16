rust
#![feature(if_let_guard, let_chains)]

fn main() {
    let xs = vec![0i32];
    if let Some(val) = xs.iter().next() && let bind_me = 5 {
        dbg!(val, bind_me);
    }
}
