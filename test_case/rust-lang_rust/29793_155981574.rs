 rust
fn main() {
    let x = Some(|x: usize, y: usize| {
        |t: bool| if t { x } else { y }
    });

    // If the following line is uncommented, it doesn't compile
    //use_(x);
}

fn use_<G, F: FnMut(usize, usize) -> G>(f: Option<F>) {
    if let Some(mut f) = f {
        let _ = f(1, 2);
    }
}
