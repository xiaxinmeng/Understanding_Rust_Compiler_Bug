 rust
pub pure fn my_from_fn<T>(n_elts: uint, op: iter::InitOp<T>) -> ~[T] {
    unsafe {
        let mut v = vec::with_capacity(n_elts);
        do vec::as_mut_buf(v) |p, _len| {
            let mut i: uint = 0u;
            while i < n_elts {
                //vec::rusti::move_val_init(&mut(*ptr::mut_offset(p, i)), op(i)); <-- change here
                *ptr::mut_offset(p, i) = move op(i); // new code
                i += 1u;
            }
        }
        vec::raw::set_len(&mut v, n_elts);
        return move v;
    }
}
