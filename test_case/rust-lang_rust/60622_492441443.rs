rust
fn run_wild<T, DL: Borked<T>>(dl: &DL) {
    dl.a::<'_, T>();
}

pub trait Borked<T> {
    fn a(&self);
}

fn main() {}
