 rust
Left => {
    let k = find_or_insert_with(&mut save.left, key, f);
    skew(save); // We can't borrow save again here because we're holding onto k
    split(save);
    k
}
