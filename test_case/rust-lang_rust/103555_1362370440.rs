rs
fn zip_map_fl<T: Copy, U: Copy, V, F: FnMut(T, U) -> V, const N: usize>(a: [T; N], b: [U; N], mut f: F) -> [V; N] {
    let mut out: [MaybeUninit<V>; N] = MaybeUninit::uninit_array();
    for i in 0..N {
        out[i] = MaybeUninit::new(f(a[i], b[i]));
    }
    unsafe {
        MaybeUninit::array_assume_init(out)
    }
}

fn zip_map_std<T, U, V, F: Fn(T, U) -> V, const N: usize>(a: [T; N], b: [U; N], f: F) -> [V; N] {
    a.zip(b).map(|(a, b)| f(a, b))
}
