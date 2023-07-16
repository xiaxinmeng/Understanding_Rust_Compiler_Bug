rust
let (foo, _guard) = self.bar.map(|bar| {
    let (value, guard) = process(bar);
    Some(value, guard)
}).unzip();
