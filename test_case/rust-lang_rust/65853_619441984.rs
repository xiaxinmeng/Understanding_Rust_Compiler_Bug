rust
fn grid<T>(w: usize, h: usize) -> Vec<Vec<T>> {
    let mut v = vec![];
    for _ in 0..w {
        v.push(Vec::with_capacity(h));
    }
    v
}

// this next fn definitely doesn't work because it doesn't have a T yet
fn is_easygoing(par: T, arg: T) -> bool {
   par == arg
}
