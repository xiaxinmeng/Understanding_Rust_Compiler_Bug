
match buf.as_slice() {
    // contents are contiguous
    Some(slice) => {
        do_things(slice);
    }
    // contents aren't contiguous
    None => {
        let (slice1, slice2) = buf.as_slices();
        do_things(slice1);
        do_things(slice2);
    }
