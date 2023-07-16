rust
match (&left, &right) {
    (l, r) => {
        if !(*l == *r) {
            panic!("...");
        }
    }
}
