rust
fn non_generic(x: u8) {
    let r = &mut x;
    RetagFnEntry(r);
    // No implicit drop terminator because `u8: Copy`
}

fn generic<T>(x: T) {
    let r = &mut x;
    RetagFnEntry(r);
    // Unconditional UB here, because the retag for the drop
    // would attempt to pop the tag of `r`
}

fn main() {
    non_generic(0_u8); // DB
    generic(0_u8); // UB
}
