rust
let c = Cell::new(Vec::new());
c.update(|v| v.push("foo"));
