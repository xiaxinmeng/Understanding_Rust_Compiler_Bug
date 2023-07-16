rust
let foo = self.bar.map(|bar| {
    let (value, guard) = process(bar);
    Some(value, guard)
});
