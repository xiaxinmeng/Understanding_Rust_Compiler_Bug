rust
fn gimme_static_mut() -> &'static mut u32 {
    match 123443 {
       ref mut x => x,
    }
}

fn main() {
    let x = gimme_static_mut();
    let y = gimme_static_mut();
    *y = 42;
    let a = *x;
    println!("a: {}", a);
}
