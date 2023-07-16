rust
fn main() {
    let f = |x: ()| x;
    let g = move |y| f(y);
    xxx();
}

fn xxx() {}
