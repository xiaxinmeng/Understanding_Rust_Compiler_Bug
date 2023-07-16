Rust
struct Range {
    start: u64,
    end: u64
}

fn next(ret: &out Option<u64>, self: &mut Range)  {
    let Range { start, end } = *self;
    let (offset, value) = if start < end {
        if let Some(next) = start.checked_add(1) {
            self.start = next;
            ret.<discriminant> = Some;
            (1, start)
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    };
    // (sic) yes this writes a "value" to an "offset".
    (ret as *mut [u64; 2])[offset] = value; // WHAT?
}
