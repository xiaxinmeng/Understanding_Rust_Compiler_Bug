rust
loop {
    match self.next() {
        None => return other.next().is_none(),
        Some(x) => match other.next() {
            None => return false,
            Some(y) => if x != y { return false },
        },
    };
}
