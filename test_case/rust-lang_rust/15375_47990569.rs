 rust
match self.a.partial_cmp(&other.a) {
    Some(Equal) => {}
    r => return r
}

match self.b.partial_cmp(&other.b) {
    Some(Equal) => {}
    r => return r,
}

...
