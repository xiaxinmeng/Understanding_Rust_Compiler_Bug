rust
fn closure_hrtb() -> impl for<'a> Fn(&'a u32)
                  -> impl Debug + 'a {
    |x| x
}
