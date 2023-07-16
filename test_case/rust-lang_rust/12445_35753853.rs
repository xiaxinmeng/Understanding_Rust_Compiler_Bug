 rust
fn transmute_pod<T: Pod, U: Pod>(t: T) -> U { unsafe { cast::transmute(t) } }
