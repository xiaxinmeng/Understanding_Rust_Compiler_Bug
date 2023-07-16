rust
loop {
    let x = match self.next() {
        None => return other.next().is_none(),
        Some(val) => val,
    };

    let y = match other.next() {
        None => return false,
        Some(val) => val,
    };

    if x != y {
        return false;
    }
}
