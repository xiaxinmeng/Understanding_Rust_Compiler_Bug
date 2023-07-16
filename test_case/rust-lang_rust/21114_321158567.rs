rust
// This is fine
fn drain_good(mut a: Vec<u32>) -> Vec<u32> {
    a.drain(..).collect::<Vec<u32>>()
}

// This is not
fn drain_bad(a: Vec<u32>) -> Vec<u32> {
    let mut a = a;
    a.drain(..).collect::<Vec<u32>>()
}

// But this is fine again.
fn drain_explicit_return(a: Vec<u32>) -> Vec<u32> {
    let mut a = a;
    return a.drain(..).collect::<Vec<u32>>();
}
