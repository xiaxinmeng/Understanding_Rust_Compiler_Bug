 rust
fn f3() {
    let mut x = 3;
    //~^ ERROR variable `x` is assigned to, but never used
    x += 4;
    //~^ ERROR value assigned to `x` is never read
}

fn f3b() {
    let mut z = 3;
    //~^ ERROR variable `z` is assigned to, but never used
    loop {
        z += 4;
    }
}
