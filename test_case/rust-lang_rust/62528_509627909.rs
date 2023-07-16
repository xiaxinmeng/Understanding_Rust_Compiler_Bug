rust
let nested: Vec<Vec<Foo>> = /* … */;
let separator: &[Foo] = /* … */;  // Previously: could only be a single &Foo
nested.join(separator)
