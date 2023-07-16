rust
if let Some((start, rest)) = slice.split_first_mut() {
    for el in rest {
        el.clone_from(&value)
    }

    *start = value;
}
